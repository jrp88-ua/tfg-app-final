use rusqlite::{ffi::SQLITE_NOTADB, Connection};
use serde::Serialize;
use ts_rs::TS;

#[derive(Default)]
pub struct AppData(Option<(String, Connection)>);

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase", tag = "type")]
#[ts(export, export_to = "../../src/lib/types/generated/")]
pub enum CloseConnectionError {
    Closing,
}

#[derive(Serialize, TS, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
#[ts(export, export_to = "../../src/lib/types/generated/")]
pub enum OpenConnectionError {
    AlreadyOpen,
    Open,
    Password,
    InvalidFile,
    UnknwonFileVersion { version: i64, supported: i64 },
    Unknown { message: Option<String> },
}

impl AppData {
    pub fn has_file(&self) -> bool {
        return self.0.is_some();
    }

    pub fn close(mut self) -> Result<AppData, CloseConnectionError> {
        match self.0 {
            Some((path, connection)) => {
                let close_result = connection.close();
                match close_result {
                    Ok(()) => Ok(AppData::default()),
                    Err((con, _)) => {
                        self.0 = Some((path.to_owned(), con));
                        Err(CloseConnectionError::Closing)
                    }
                }
            }
            None => Ok(AppData::default()),
        }
    }

    pub fn open(&mut self, file: String, pass: String) -> Result<(), OpenConnectionError> {
        if self.has_file() {
            return Err(OpenConnectionError::AlreadyOpen);
        }

        let conn = Connection::open(file.clone()).map_err(|_| OpenConnectionError::Open)?;
        verify_connection(&conn, pass)?;

        self.0 = Some((file, conn));

        Ok(())
    }
}
#[derive(Debug)]
struct MepMeta {
    file_version: i64,
}

impl MepMeta {
    fn from(value: &rusqlite::Row<'_>) -> Result<MepMeta, rusqlite::Error> {
        Ok(MepMeta {
            file_version: value.get("file_version")?,
        })
    }
}

fn verify_connection(conn: &Connection, pass: String) -> Result<(), OpenConnectionError> {
    conn.pragma_update(None, "key", &pass).unwrap();
    conn.pragma_update(None, "legacy", 4).unwrap();

    let result = conn.query_row("SELECT * FROM mep_meta;", [], |row| MepMeta::from(row));
    println!("result={:?}", result);

    if let Err(err) = result {
        match err {
            rusqlite::Error::SqliteFailure(err, message) => {
                if let Some(message) = message {
                    if message == "no such table: mep_meta" {
                        return create_database(conn);
                    } else {
                        return Err(OpenConnectionError::Unknown {
                            message: Some(message),
                        });
                    }
                } else if err.code == rusqlite::ErrorCode::NotADatabase {
                    return Err(OpenConnectionError::Password);
                }
                return Err(OpenConnectionError::Unknown { message });
            }
            rusqlite::Error::InvalidColumnName(_) => return Err(OpenConnectionError::InvalidFile),
            _ => return Err(OpenConnectionError::Unknown { message: None }),
        };
    }
    let meta = result.unwrap();
    if meta.file_version != 1 {
        return Err(OpenConnectionError::UnknwonFileVersion {
            version: meta.file_version,
            supported: 1,
        });
    }

    Ok(())
}

fn create_database(conn: &Connection) -> Result<(), OpenConnectionError> {
    Ok(())
}

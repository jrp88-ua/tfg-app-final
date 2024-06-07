// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod event;
mod ipc;
mod models;
mod state;

use std::sync::{Arc, Mutex};

use log::{info, LevelFilter};
use state::app_data::{self, AppData};
use tauri_plugin_log::LogTarget;

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

#[tokio::main]
async fn main() -> Result<(), ()> {
    app_data::AppData::default()
        .open("D:\\test.sqlite".to_owned(), "juan".to_owned())
        .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::ipc::import::start_examinee_import_process,
            crate::ipc::import::perform_examinee_import,
            crate::ipc::import::cancel_examinee_import,
        ])
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .with_colors(tauri_plugin_log::fern::colors::ColoredLevelConfig::default())
                .level_for("tauri", log::LevelFilter::Info)
                .level(LevelFilter::Debug)
                .build(),
        )
        .manage(Arc::new(Mutex::new(
            Option::<Vec<ipc::import::SheetData>>::None,
        )))
        .manage(Arc::new(Mutex::new(AppData::default())))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

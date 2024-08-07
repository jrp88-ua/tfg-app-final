// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(unused_extern_crates)]

mod event;
mod ipc;
mod models;
mod storage;

use std::{
    env,
    sync::{Arc, Mutex},
};

use log::LevelFilter;
use tauri_plugin_log::LogTarget;

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

#[tokio::main]
async fn main() -> Result<(), ()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::ipc::import::start_examinee_import_process,
            crate::ipc::import::perform_examinee_import,
            crate::ipc::import::cancel_examinee_import,
            crate::ipc::export::export_assignment,
            crate::ipc::open_file::open_file_from_open_with,
            crate::ipc::open_file::load_file,
            crate::ipc::save_file::save_file,
            crate::ipc::open::open_file,
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::Mutex;

use log::{info, trace, warn};
use tauri::State;
use tauri_plugin_log::LogTarget;

pub mod dbc;

#[derive(Debug)]
struct AppState(Mutex<HashMap<String, DbcState>>);

#[derive(Debug)]
struct DbcState {
    name: String,
    path: String,
    dbc: Option<dbc::dbc::DBC>,
}

#[tauri::command]
fn file_load(path: String, state: State<'_, AppState>) {
    info!("file_load func called: {}", path);

    let mut stat = state.0.lock().unwrap();

    // todo: parse file name
    let has_key = stat.contains_key(&path);
    if has_key {
        info!("cannot insert data, beacuse key is already existed.");
    } else {
        stat.insert(
            path,
            DbcState {
                name: "name".to_string(),
                path: "path".to_string(),
                dbc: Option::None,
            },
        );
    }

    info!("{:?}", stat);
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(HashMap::new())))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![file_load])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use log::{debug, info, error, trace, warn};
use serde_json::error;
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

impl DbcState {
    fn new(name: String, path: String) -> Result<Self, String> {
        let raw_dbc =
            fs::read_to_string(path.clone()).expect("Should have been able to read the file");
        info!("raw dbc: {}", raw_dbc);

        let dbc = dbc::parser::parse_dbc(&raw_dbc);

        match dbc {
            Ok(dbc) => {
                Ok(Self {
                    name,
                    path,
                    dbc: Some(dbc.1),
                })
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}

#[tauri::command]
fn file_load(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let name = path.split("/").last().unwrap().to_string();
    info!("file name: {}", name);

    let mut stat = state.0.lock().unwrap();
    let has_key = stat.contains_key(&name);

    if has_key {
        info!("cannot insert data, beacuse key is already existed.");
        return Err(name);
    } else {
        info!("inserting new data");

        let dbc_state = DbcState::new(name.clone(), path.clone());
        match dbc_state {
            Ok(dbc_state) => {
                stat.insert(name.clone(), dbc_state);
            }
            Err(e) => {
                error!("{}", e);
                return Err(name);
            }
        }
    }
    debug!("DBC state: {:?}", stat);

    Ok(name)
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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use log::{debug, error, info, trace, warn};

use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use tauri::State;
use tauri_plugin_log::LogTarget;

pub mod dbc;

#[derive(Debug)]
struct AppState(Mutex<HashMap<String, DbcState>>);

#[derive(Debug, Clone)]
struct DbcState {
    name: String,
    path: String,
    dbc: dbc::dbc::DBC,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessagesInfo {
    name: String,
    cnt: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct DbcInfo {
    file_name: String,
    version: String,
    symbols: Vec<String>,
    nodes: Vec<String>,
    messages: Vec<MessagesInfo>,
    comments: Vec<Vec<String>>,
}

impl DbcState {
    fn new(name: String, path: String) -> Result<Self, String> {
        let raw_dbc =
            fs::read_to_string(path.clone()).expect("Should have been able to read the file");
        info!("raw dbc: {}", raw_dbc);

        let dbc = dbc::parser::parse_dbc(&raw_dbc);

        match dbc {
            Ok(dbc) => Ok(Self {
                name,
                path,
                dbc: dbc.1,
            }),
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

#[tauri::command]
fn get_dbc_info(dbc: String, state: State<'_, AppState>) -> Result<String, String> {
    info!("get_dbc_info(): {}", dbc);

    let stat = state.0.lock().unwrap();
    let has_key = stat.contains_key(&dbc);

    match has_key {
        true => {
            let d = stat[&dbc].clone();

            let dbc_info = DbcInfo {
                file_name: d.name,
                version: d.dbc.version.0,
                symbols: d
                    .dbc
                    .new_symbol
                    .into_iter()
                    .map(|symbol| symbol.0)
                    .collect(),
                nodes: d.dbc.nodes.into_iter().map(|node| node.0).collect(),
                messages: d
                    .dbc
                    .messages
                    .into_iter()
                    .map(|message| MessagesInfo {
                        name: message.name,
                        cnt: message
                            .signals
                            .map(|signal| signal.len())
                            .unwrap_or_default(),
                    })
                    .collect(),
                comments: d
                    .dbc
                    .comments
                    .into_iter()
                    .map(|comment| match comment {
                        dbc::dbc::Comment::Normal(cmt) => vec![cmt],
                        dbc::dbc::Comment::Node { name, comment } => vec![name, comment],
                        dbc::dbc::Comment::Message { id, comment } => vec![id.to_string(), comment],
                        dbc::dbc::Comment::Signal { id, name, comment } => {
                            vec![id.to_string(), name, comment]
                        }
                    })
                    .collect(),
            };

            match serde_json::to_string(&dbc_info) {
                Ok(val) => {
                    println!("{}", val);
                    return Ok(val);
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }
        _ => (),
    }

    Err(dbc)
}

#[tauri::command]
fn get_message_info(state: State<'_, AppState>) -> Result<String, String> {
    Ok(String::from("x"))
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(HashMap::new())))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            file_load,
            get_dbc_info,
            get_message_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

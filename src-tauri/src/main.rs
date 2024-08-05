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
            Ok(dbc) => Ok(Self {
                name,
                path,
                dbc: Some(dbc.1), 
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
fn get_summary_info(path: String, state: State<'_, AppState>) -> Result<String, String> {
    info!("get summary info: {}", path);
    let john = json!({
        "file_name": "CANDBC_FILE_.dbc",
        "version": "0.1.0",
        "symbols": ["CM_", "BA_", "VAL_", "CAT_"],
        "nodes": ["K16_BECM", "K114B_HPCM", "T18_BatteryCharger"],
        "messages_info": [
            {
                "message_name": "WebData_1840",
                "message_cnt": 3
            },
            {
                "message_name": "Battery_Module_1",
                "message_cnt": 10
            },
            {
                "message_name": "Battery_Module_2",
                "message_cnt": 15
            }
        ],
        "comments": [
            ["Imported file _honda_common.dbc starts here"],
            ["BO_", "1840", "Some Message comment"],
            [
                "SG_",
                "1840",
                "Signal_4",
                "asaklfjlsdfjlsdfglsHH?=(%)/&KKDKFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv"
            ],
            ["SG_", "5", "TestSigLittleUnsigned1", "0943503450KFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv"],
            ["BU_", "K17_EBCM", "Electronic Brake Control Module"]
        ]
    });

    Ok(john.to_string())
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
        .invoke_handler(tauri::generate_handler![file_load, get_summary_info, get_message_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

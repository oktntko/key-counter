// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keyboard_listener;
use std::thread;
use tauri::Manager;
use tauri::Emitter;
use log::info;

#[derive(Clone, serde::Serialize)]
struct Payload {
    mode: String,
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    info!("The name param is {}", name);

    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            let wv = app.get_webview_window("main").unwrap();

            thread::spawn(move || {
                keyboard_listener::run_listener(move |s: &str, s1: &str| {
                    info!("The name param is {} {}", s, s1);
                    if let Err(err) = wv.emit(
                        "keypress",
                        Payload {
                            mode: String::from(s),
                            message: String::from(s1),
                        },
                    ) {
                        eprintln!("Error while emitting event: {:?}", err);
                    }
                })
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

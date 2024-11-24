// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use log::info;
use std::thread;
use rdev::{listen, Event, EventType};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .setup(move |_| {
            let callback = move |event: Event| {
                match event.name {
                    Some(string) => {
                        info!("rdev callback Some: {}", string);
                    }
                    None => {
                        match event.event_type {
                            EventType::KeyPress(key) => {
                                let key_str = format!("{:?}", key);
                                info!("rdev callback KeyPress: {} {:?}", key_str, key);
                            }
                            EventType::KeyRelease(key) => {
                                let key_str = format!("{:?}", key);
                                info!("rdev callback KeyRelease: {} {:?}", key_str, key);
                            }
                            EventType::ButtonPress { .. } => {
                                // Ignore ButtonPress event type マウスボタン
                            }
                            EventType::ButtonRelease { .. } => {
                                // Ignore ButtonRelease event type マウスボタン
                            }
                            EventType::MouseMove { .. } => {
                                // Ignore MouseMove event type
                            }
                            EventType::Wheel { .. } => {
                                // Ignore Wheel event type
                            }
                        }
                    }
                }
            };

            thread::spawn(move || {
                if let Err(error) = listen(callback) {
                    info!("Error: {:?}", error);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

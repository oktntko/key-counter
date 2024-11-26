// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use log::info;
use rdev::{listen, Event, EventType};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool};

const DB_URL: &str = "sqlite:key-counter.db";

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    create_db_if_not_exists().await;

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
            let emit_event = move || async {
                let db = connect_db().await;
                sqlx::query(
                    "INSERT INTO counter (
                      date
                    , time
                    , key
                    , count
                    ) VALUES (
                      $1
                    , $2
                    , $3
                    , 1
                    ) ON CONFLICT(date, time, key) DO UPDATE SET
                      count = count + 1;",
                )
                .bind("date")
                .bind("time")
                .bind("key")
                .execute(&db)
                .await
                .unwrap();
            };

            let callback = move |event: Event| {
                match event.name {
                    Some(string) => {
                        info!("rdev callback Some: {}", string);
                    }
                    None => {
                        #[allow(unused_must_use)] // emit_event no await
                        match event.event_type {
                            EventType::KeyPress(key) => {
                                let key_str = format!("{:?}", key);
                                info!("rdev callback KeyPress: {} {:?}", key_str, key);
                                emit_event();
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

            tokio::spawn(async move {
                if let Err(error) = listen(callback) {
                    info!("Error: {:?}", error);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn create_db_if_not_exists() {
    if !sqlx::Sqlite::database_exists(&DB_URL).await.unwrap() {
        sqlx::Sqlite::create_database(&DB_URL).await.unwrap();
    }

    let db = connect_db().await;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS
counter (
    id      INT     PRIMARY KEY
  , date    TEXT    NOT NULL
  , time    TEXT    NOT NULL
  , key     TEXT    NOT NULL
  , count   INT     NOT NULL DEFAULT 0
  , UNIQUE (date, time, key) ON CONFLICT REPLACE
)",
    )
    .execute(&db)
    .await
    .unwrap();
}

async fn connect_db() -> SqlitePool {
    return SqlitePool::connect(&DB_URL).await.unwrap();
}

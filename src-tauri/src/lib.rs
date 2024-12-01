// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use chrono::{
    prelude::{DateTime, Local},
    Timelike,
};
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
            let emit_event = move |date: String, time: String, key_str: String| async {
                info!("emit_event begin");

                let db = connect_db().await;
                sqlx::query(
                    "
                        INSERT INTO counter (
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
                            count = count + 1
                      ",
                )
                .bind(date)
                .bind(time)
                .bind(key_str)
                .execute(&db)
                .await
                .unwrap();

                info!("emit_event end");
            };

            let callback = move |event: Event| {
                let dt: DateTime<Local> = event.time.clone().into();
                let date = dt.format("%Y-%m-%d").to_string();
                let h: u32 = dt.hour();
                // TODO: 1h, 3h, 6h, 12h, 24h で設定できるようにする
                let fixed_hour = if h < 3 {
                    0
                } else if h < 6 {
                    3
                } else if h < 9 {
                    6
                } else if h < 12 {
                    9
                } else if h < 15 {
                    12
                } else if h < 18 {
                    15
                } else if h < 21 {
                    18
                } else {
                    21
                };
                let time = format!("{:2}:00:00", fixed_hour).to_string();
                match event.name {
                    Some(key_str) => {
                        info!("Some: {} {} [{}]", date, time, key_str);
                        tokio::spawn(emit_event(date, time, key_str));
                    }
                    None => {
                        match event.event_type {
                            EventType::KeyRelease(key) => {
                                let key_str = format!("{:?}", key);
                                // TODO: Ctrl や Shift などの 実際の文字が取得できないキーに対応する
                                // key_code の方を入力するとか
                                info!("KeyRelease: {} {} [{}]", date, time, key_str);
                                tokio::spawn(emit_event(date, time, key_str));
                            }
                            _ => {
                                // Ignore event
                                // EventType::KeyPress 長押しするとイベントが発火し続けるため無視する
                                // EventType::ButtonPress マウスボタン
                                // EventType::ButtonRelease マウスボタン
                                // EventType::MouseMove
                                // EventType::Wheel
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
        .invoke_handler(tauri::generate_handler![load_counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn create_db_if_not_exists() {
    if !sqlx::Sqlite::database_exists(&DB_URL).await.unwrap() {
        sqlx::Sqlite::create_database(&DB_URL).await.unwrap();
    }

    let db = connect_db().await;
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS counter (
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

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
struct Counter {
    date: String,
    time: String,
    key: String,
    count: String,
}

#[tauri::command]
async fn load_counter() -> Result<Vec<Counter>, String> {
    let db = connect_db().await;
    let counters = sqlx::query_as::<_, Counter>(
        "
        SELECT
              date
            , time
            , key
            , SUM(count) AS count
        FROM
            counter
        GROUP BY
              date
            , time
            , key
        ",
    )
    .fetch_all(&db)
    .await
    .map_err(|e| format!("Failed to get counters {}", e))?;

    info!("load_counter: {:?}", counters);
    Ok(counters)
}

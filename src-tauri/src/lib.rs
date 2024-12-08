use std::collections::HashMap;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use chrono::{
    prelude::{DateTime, Local},
    Timelike,
};
use log::info;
use rdev::{listen, Event, EventType, Key};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool};
use tauri_plugin_autostart::MacosLauncher;

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
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .invoke_handler(tauri::generate_handler![
            start_listen,
            load_key_heatmap,
            list_counter
        ])
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

#[tauri::command]
async fn start_listen() -> Result<(), String> {
    info!("start_listen begin");
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
                let trimed = key_str.trim().to_string();
                if !trimed.is_empty() {
                    info!("Some: {} {} [{}]", date, time, trimed);
                    // TODO: 文字コード US（ユニット区切り）までデータベースに登録してしまう
                    // BS（後退）、ESC（エスケープ） を登録してしまう
                    tokio::spawn(emit_event(date, time, key_str.to_lowercase()));
                }
            }
            None => {
                match event.event_type {
                    EventType::KeyRelease(key) => {
                        let key_str = format!("{:?}", key);
                        // TODO: Ctrl や Shift などの 実際の文字が取得できないキーに対応する
                        // key_code の方を入力するとか
                        match key {
                            Key::BackQuote
                            | Key::Num1
                            | Key::Num2
                            | Key::Num3
                            | Key::Num4
                            | Key::Num5
                            | Key::Num6
                            | Key::Num7
                            | Key::Num8
                            | Key::Num9
                            | Key::Num0
                            | Key::Minus
                            | Key::Equal
                            | Key::KeyQ
                            | Key::KeyW
                            | Key::KeyE
                            | Key::KeyR
                            | Key::KeyT
                            | Key::KeyY
                            | Key::KeyU
                            | Key::KeyI
                            | Key::KeyO
                            | Key::KeyP
                            | Key::LeftBracket
                            | Key::RightBracket
                            | Key::KeyA
                            | Key::KeyS
                            | Key::KeyD
                            | Key::KeyF
                            | Key::KeyG
                            | Key::KeyH
                            | Key::KeyJ
                            | Key::KeyK
                            | Key::KeyL
                            | Key::SemiColon
                            | Key::Quote
                            | Key::BackSlash
                            | Key::IntlBackslash
                            | Key::KeyZ
                            | Key::KeyX
                            | Key::KeyC
                            | Key::KeyV
                            | Key::KeyB
                            | Key::KeyN
                            | Key::KeyM
                            | Key::Comma
                            | Key::Dot
                            | Key::Slash
                            | Key::Insert
                            | Key::KpMinus
                            | Key::KpPlus
                            | Key::KpMultiply
                            | Key::KpDivide
                            | Key::Kp0
                            | Key::Kp1
                            | Key::Kp2
                            | Key::Kp3
                            | Key::Kp4
                            | Key::Kp5
                            | Key::Kp6
                            | Key::Kp7
                            | Key::Kp8
                            | Key::Kp9 => {
                                // Some で入力された文字を取っているので無視する
                            }
                            _ => {
                                info!("KeyRelease: {} {} [{}]", date, time, key_str);
                                tokio::spawn(emit_event(date, time, key_str.to_lowercase()));
                            }
                        }
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

    if let Err(error) = listen(callback) {
        info!("Error: {:?}", error);
    }

    info!("start_listen finish");
    Ok(())
}

#[tauri::command]
async fn load_key_heatmap() -> Result<HashMap<String, f64>, String> {
    #[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
    struct Row {
        key: String,
        count: i64,
    }

    let db = connect_db().await;
    let rows = sqlx::query_as::<_, Row>(
        "
        SELECT
              key
            , SUM(count) AS count
        FROM
            counter
        GROUP BY
              key
        ",
    )
    .fetch_all(&db)
    .await
    .map_err(|e| format!("Failed to get counters {}", e))?;

    let max: i64 = rows.iter().map(|x| x.count).max().unwrap();
    let heatmap: HashMap<String, f64> = rows.into_iter().fold(HashMap::new(), |mut acc, cur| {
        acc.insert(cur.key, cur.count as f64 / max as f64);
        acc
    });

    info!("load_key_heatmap: {:?} {}", heatmap, max);
    Ok(heatmap)
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
struct Counter {
    date: String,
    time: String,
    key: String,
    count: i64,
}

#[tauri::command]
async fn list_counter() -> Result<Vec<Counter>, String> {
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
              key
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

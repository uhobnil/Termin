// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod notify;
mod state;
mod tray;

pub mod commands;
pub mod database;
pub mod errors;
pub mod setup;

use crate::commands::schedule::*;
use crate::state::ScheduleState;
use sea_orm::DatabaseConnection;
use serde_json::from_value;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_store::StoreExt;

use std::time::Duration;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let schedule_state = ScheduleState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_schedule_by_month,
            get_schedule_today,
            create_schedule,
            update_schedule,
            delete_schedule
        ])
        .manage(schedule_state)
        .setup(|app| {
            // 初始化配置
            let app_handle = app.handle().clone();
            config::init(&app_handle);

            //数据库
            let app_handle = app.handle().clone();
            app.manage(setup::get_database_pool(&app_handle));

            // 隐藏docker图标
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_autostart::init(
                        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                        Some(vec!["--minimized"]),
                    ))
                    .expect("Failed to initialize autostart plugin");
            }

            let autostart_manager = app.autolaunch();
            let store = app.get_store("store.json").unwrap();
            let auto_start = from_value(store.get("autoStart").unwrap()).unwrap();
            if auto_start {
                let _ = autostart_manager.enable();
            } else {
                let _ = autostart_manager.disable();
            }

            let _ = tray::create_tray(app);

            tauri::async_runtime::spawn(async move {
                // 启动时刷新schedules
                let state = app_handle.state::<ScheduleState>();
                let db = app_handle.state::<DatabaseConnection>();
                if let Err(e) = state.refresh(&db).await {
                    eprintln!("Failed to refresh schedules: {}", e);
                }
                loop {
                    let _ = notify::notify(&app_handle).await;
                    std::thread::sleep(Duration::from_secs(1))
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                #[cfg(not(target_os = "macos"))]
                {
                    window.hide().unwrap();
                }

                #[cfg(target_os = "macos")]
                {
                    tauri::AppHandle::hide(&window.app_handle()).unwrap();
                }
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

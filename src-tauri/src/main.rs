// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::{Target, TargetKind};

mod commands;
mod manager;
mod models;
mod state;

mod utils;

pub async fn setup(_app: &mut tauri::App) -> anyhow::Result<()> {
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(state::AppState::default())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                setup(app).await.unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_config,
            commands::save_app_config,
            commands::get_clipboard_text,
            commands::set_clipboard_text,
            commands::get_clipboard_rtf,
            commands::set_clipboard_rtf,
            commands::get_clipboard_html,
            commands::set_clipboard_html,
            commands::get_clipboard_image,
            commands::set_clipboard_image,
            commands::get_clipboard_files,
            commands::set_clipboard_files,
            commands::get_clipboard_format,
            commands::set_clipboard_format,
            commands::guess_lang,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

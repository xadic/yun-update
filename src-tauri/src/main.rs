#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod config;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu};
use crate::db::util::backup_acdb;

#[tokio::main]
async fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .setup(|app| {
            let id = app.listen_global("update", |event| {
                println!("you got it, {:?}", event.payload());
            });
            // app.unlisten(id);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![backup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn backup() {
    backup_acdb().await.unwrap();
}

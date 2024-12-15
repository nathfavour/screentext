// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::tray::{SystemTray, SystemTrayEvent, SystemTrayMenu, CustomMenuItem};

fn main() {
    // Create a "Quit" menu item for the tray
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    tauri::Builder::default()
        .setup(|app| {
            let tray = SystemTray::new().with_menu(tray_menu);
            app.system_tray(tray)?;
            let _window = app.handle().get_webview_window("main").unwrap();
            Ok(())
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                if id.as_str() == "quit" {
                    std::process::exit(0);
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


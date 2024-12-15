// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
  SystemTrayEvent, CustomMenuItem,
};

fn main() {
    // Create a "Quit" menu item for the tray
    let quit_i = MenuItem::with_id("quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(&[&quit_i]).unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .build(app)?;
            app.system_tray(tray);
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


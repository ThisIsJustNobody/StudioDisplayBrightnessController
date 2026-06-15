mod commands;
mod studio_display;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tray::setup_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_displays,
            commands::set_brightness
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

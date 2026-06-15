use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Manager,
};

use crate::studio_display::{brightness, protocol::brightness_from_percent};

const MENU_OPEN: &str = "open";
const MENU_BRIGHTNESS_25: &str = "brightness_25";
const MENU_BRIGHTNESS_50: &str = "brightness_50";
const MENU_BRIGHTNESS_75: &str = "brightness_75";
const MENU_BRIGHTNESS_100: &str = "brightness_100";
const MENU_QUIT: &str = "quit";

pub fn setup_tray(app: &mut App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, MENU_OPEN, "打开", true, None::<&str>)?;
    let brightness_25 = MenuItem::with_id(app, MENU_BRIGHTNESS_25, "亮度 25%", true, None::<&str>)?;
    let brightness_50 = MenuItem::with_id(app, MENU_BRIGHTNESS_50, "亮度 50%", true, None::<&str>)?;
    let brightness_75 = MenuItem::with_id(app, MENU_BRIGHTNESS_75, "亮度 75%", true, None::<&str>)?;
    let brightness_100 =
        MenuItem::with_id(app, MENU_BRIGHTNESS_100, "亮度 100%", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, MENU_QUIT, "退出", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[
            &open,
            &brightness_25,
            &brightness_50,
            &brightness_75,
            &brightness_100,
            &quit,
        ],
    )?;

    let mut tray = TrayIconBuilder::new()
        .tooltip("Studio Display Brightness")
        .menu(&menu)
        .on_menu_event(|app, event| {
            let id = event.id().as_ref();
            if id == MENU_OPEN {
                show_main_window(app);
            } else if let Some(percent) = preset_percent_from_menu_id(id) {
                set_tray_brightness(percent);
            } else if id == MENU_QUIT {
                app.exit(0);
            }
        });

    if let Some(icon) = app.default_window_icon().cloned() {
        tray = tray.icon(icon);
    }

    tray.build(app)?;

    Ok(())
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if let Err(error) = window.show() {
            eprintln!("tray open failed: {error}");
        }
        if let Err(error) = window.set_focus() {
            eprintln!("tray focus failed: {error}");
        }
    }
}

fn set_tray_brightness(percent: u8) {
    let value = brightness_from_percent(percent);
    if let Err(error) = brightness::set_brightness(value) {
        eprintln!("tray brightness failed: {} ({error})", error.user_message());
    }
}

fn preset_percent_from_menu_id(id: &str) -> Option<u8> {
    match id {
        MENU_BRIGHTNESS_25 => Some(25),
        MENU_BRIGHTNESS_50 => Some(50),
        MENU_BRIGHTNESS_75 => Some(75),
        MENU_BRIGHTNESS_100 => Some(100),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_tray_preset_menu_ids_to_percentages() {
        assert_eq!(preset_percent_from_menu_id("brightness_25"), Some(25));
        assert_eq!(preset_percent_from_menu_id("brightness_50"), Some(50));
        assert_eq!(preset_percent_from_menu_id("brightness_75"), Some(75));
        assert_eq!(preset_percent_from_menu_id("brightness_100"), Some(100));
        assert_eq!(preset_percent_from_menu_id("open"), None);
        assert_eq!(preset_percent_from_menu_id("quit"), None);
    }
}

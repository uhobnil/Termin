use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};

pub fn create_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Create Tray Icon with menu
    let dashboard_i = MenuItem::with_id(app, "dashboard", "Home", true, None::<&str>)?;
    let relaunch_i = MenuItem::with_id(app, "relaunch", "Restart", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&dashboard_i, &relaunch_i, &quit_i])?;
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                println!("left click pressed and released");
                // in this example, let's show and focus the main window when the tray is clicked
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_skip_taskbar(false);
                    let _ = window.set_focus();
                    // print the window title
                    println!("window title: {}", window.title().unwrap());
                }
            }
            _ => {
                println!("unhandled event {event:?}");
            }
        })
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            "relaunch" => {
                println!("quit menu item was clicked");
                app.restart();
            }
            "dashboard" => {
                // in this example, let's show and focus the main window when the tray is clicked
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_skip_taskbar(false);
                    let _ = window.set_focus();
                }
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;

    Ok(())
}

use crate::{
    placement::{Rect, initial_bounds},
    settings::Store,
};
use std::sync::Mutex;
use tauri::{
    Manager,
    menu::{Menu, MenuItem},
};
pub type SettingsState = Mutex<Store>;
#[derive(Default)]
pub struct OverlayState {
    pub passive: Mutex<bool>,
}

pub fn place(window: &tauri::WebviewWindow) -> Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .or(window.primary_monitor().map_err(|e| e.to_string())?)
        .ok_or("No monitor available")?;
    let area = monitor.work_area();
    let b = initial_bounds(
        Rect {
            x: area.position.x,
            y: area.position.y,
            width: area.size.width,
            height: area.size.height,
        },
        monitor.scale_factor(),
    )?;
    window
        .set_size(tauri::PhysicalSize::new(b.width, b.height))
        .map_err(|e| e.to_string())?;
    window
        .set_position(tauri::PhysicalPosition::new(b.x, b.y))
        .map_err(|e| e.to_string())
}

pub fn set_passive(app: &tauri::AppHandle, passive: bool) -> Result<(), String> {
    let window = app.get_webview_window("notch").ok_or("Missing notch")?;
    let state = app.state::<OverlayState>();
    let previous = *state.passive.lock().map_err(|e| e.to_string())?;
    window
        .set_ignore_cursor_events(passive)
        .map_err(|e| e.to_string())?;
    if let Err(error) = window.set_focusable(!passive) {
        window
            .set_ignore_cursor_events(previous)
            .map_err(|rollback| format!("{error}; input rollback failed: {rollback}"))?;
        return Err(error.to_string());
    }
    *state.passive.lock().map_err(|e| e.to_string())? = passive;
    Ok(())
}

pub fn menu(app: &tauri::AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let restore = MenuItem::with_id(app, "restore", "Restaurar interação", true, None::<&str>)?;
    let passive = MenuItem::with_id(
        app,
        "passive",
        "Click-through (restaure pelo tray)",
        true,
        None::<&str>,
    )?;
    let top = MenuItem::with_id(app, "top", "Alternar sempre no topo", true, None::<&str>)?;
    let exit = MenuItem::with_id(app, "quit", "Encerrar LKOS", true, None::<&str>)?;
    Menu::with_items(app, &[&restore, &passive, &top, &exit])
}

pub fn action(app: &tauri::AppHandle, id: &str) -> Result<(), String> {
    let window = app.get_webview_window("notch").ok_or("Missing notch")?;
    match id {
        "quit" => app.exit(0),
        "restore" => {
            set_passive(app, false)?;
            window.show().map_err(|e| e.to_string())?;
        }
        "passive" => set_passive(app, true)?,
        "top" => {
            let state = app.state::<SettingsState>();
            let mut store = state.lock().map_err(|e| e.to_string())?;
            let old = store.current.always_on_top;
            let mut next = store.current.clone();
            next.always_on_top = !old;
            window
                .set_always_on_top(next.always_on_top)
                .map_err(|e| e.to_string())?;
            if let Err(error) = store.save(next) {
                window
                    .set_always_on_top(old)
                    .map_err(|rollback| format!("{error}; topmost rollback failed: {rollback}"))?;
                return Err(error);
            }
        }
        _ => return Err("Unknown overlay action".into()),
    }
    Ok(())
}

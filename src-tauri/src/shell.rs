use crate::settings::{Settings, Store};
use serde::Serialize;
use std::sync::Mutex;
use tauri::{
    Emitter, Manager, State,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

type SettingsState = Mutex<Store>;

#[tauri::command]
fn read_settings(state: State<'_, SettingsState>) -> Result<Settings, String> {
    Ok(state.lock().map_err(|e| e.to_string())?.current.clone())
}

#[derive(Clone, Serialize)]
struct ProbeEvent {
    id: String,
    r#type: &'static str,
    source: &'static str,
    timestamp: u128,
    priority: u8,
    payload: u32,
}

#[tauri::command]
fn run_probe(app: tauri::AppHandle, state: State<'_, SettingsState>) -> Result<Settings, String> {
    let mut store = state.lock().map_err(|e| e.to_string())?;
    let mut next = store.current.clone();
    next.probe_count = next
        .probe_count
        .checked_add(1)
        .ok_or("Probe counter exhausted")?;
    store.save(next.clone())?;
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis();
    app.emit_to(
        "notch",
        "foundation-probe",
        ProbeEvent {
            id: format!("probe-{timestamp}-{}", next.probe_count),
            r#type: "foundation.probe",
            source: "native.fixture",
            timestamp,
            priority: 0,
            payload: next.probe_count,
        },
    )
    .map_err(|e| e.to_string())?;
    Ok(next)
}

#[tauri::command]
fn quit(app: tauri::AppHandle) {
    app.exit(0);
}

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_settings, run_probe, quit])
        .setup(|app| {
            let store = Store::open(app.path().app_config_dir()?.join("settings.json"))
                .map_err(std::io::Error::other)?;
            let topmost = store.current.always_on_top;
            app.manage(Mutex::new(store));
            let window = app
                .get_webview_window("notch")
                .ok_or("Missing notch window")?;
            window.set_always_on_top(topmost)?;
            if let Some(monitor) = window.primary_monitor()? {
                let area = monitor.work_area();
                let scale = monitor.scale_factor();
                let x = area.position.x + (area.size.width as f64 - 520.0 * scale).max(0.0) as i32;
                let y = area.position.y + (48.0 * scale) as i32;
                window.set_position(tauri::PhysicalPosition::new(x, y))?;
            }
            let restore =
                MenuItem::with_id(app, "restore", "Restaurar interação", true, None::<&str>)?;
            let passive = MenuItem::with_id(
                app,
                "passive",
                "Click-through (restaure pelo tray)",
                true,
                None::<&str>,
            )?;
            let top = MenuItem::with_id(app, "top", "Alternar sempre no topo", true, None::<&str>)?;
            let exit = MenuItem::with_id(app, "quit", "Encerrar LKOS", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&restore, &passive, &top, &exit])?;
            let mut rgba = Vec::with_capacity(16 * 16 * 4);
            for _ in 0..16 * 16 {
                rgba.extend([184, 231, 218, 255]);
            }
            TrayIconBuilder::new()
                .tooltip("LKOS — Foundation")
                .icon(tauri::image::Image::new_owned(rgba, 16, 16))
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                        return;
                    }
                    let Some(window) = app.get_webview_window("notch") else {
                        return;
                    };
                    let result: Result<(), String> = (|| {
                        match event.id.as_ref() {
                            "restore" => {
                                window
                                    .set_ignore_cursor_events(false)
                                    .map_err(|e| e.to_string())?;
                                window.show().map_err(|e| e.to_string())?;
                            }
                            "passive" => window
                                .set_ignore_cursor_events(true)
                                .map_err(|e| e.to_string())?,
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
                                    let _ = window.set_always_on_top(old);
                                    return Err(error);
                                }
                            }
                            _ => {}
                        }
                        Ok(())
                    })();
                    if let Err(error) = result {
                        eprintln!("LKOS tray: {error}");
                    }
                })
                .build(app)?;
            window.show()?;
            Ok(())
        })
        .run(tauri::generate_context!())
}

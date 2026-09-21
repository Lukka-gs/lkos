use crate::settings::{Settings, Store};
use serde::Serialize;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};

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
    let builder = tauri::Builder::default()
        // Register before setup: secondary launches must not open the settings store.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let handle = app.clone();
            // Startup itself shows the notch if a reopen arrives before state is ready.
            if let Err(error) = app.run_on_main_thread(move || {
                if handle.try_state::<crate::overlay::OverlayState>().is_some()
                    && let Err(error) = crate::overlay::action(&handle, "restore")
                {
                    eprintln!("LKOS reopen failed: {error}");
                }
            }) {
                eprintln!("LKOS reopen dispatch failed: {error}");
            }
        }))
        .on_menu_event(|app, event| {
            let id = event.id.as_ref();
            if !["restore", "passive", "top", "quit"].contains(&id) {
                return;
            }
            let result = crate::overlay::action(app, id);
            #[cfg(debug_assertions)]
            let _ = app.emit_to(
                "overlay-lab",
                "overlay-observation",
                format!("menu:{id} {}", if result.is_ok() { "ok" } else { "error" }),
            );
            if let Err(error) = result {
                eprintln!("LKOS tray: {error}");
            }
        })
        .on_window_event(|window, event| {
            #[cfg(debug_assertions)]
            if let tauri::WindowEvent::Focused(focused) = event {
                let _ = window.app_handle().emit_to(
                    "overlay-lab",
                    "overlay-observation",
                    format!("focus:{}={focused}", window.label()),
                );
            }
            if window.label() == "notch"
                && matches!(event, tauri::WindowEvent::ScaleFactorChanged { .. })
                && let Some(notch) = window.app_handle().get_webview_window("notch")
                && let Err(error) = crate::overlay::place(&notch)
            {
                eprintln!("LKOS DPI placement: {error}");
            }
            #[cfg(debug_assertions)]
            if window.label() == "overlay-lab"
                && matches!(event, tauri::WindowEvent::CloseRequested { .. })
                && let Err(error) = crate::overlay::set_passive(window.app_handle(), false)
            {
                eprintln!("LKOS lab restore: {error}");
            }
        });
    #[cfg(debug_assertions)]
    let builder = builder.invoke_handler(tauri::generate_handler![
        read_settings,
        run_probe,
        quit,
        crate::lab::lab_action,
        crate::lab::lab_snapshot
    ]);
    #[cfg(not(debug_assertions))]
    let builder = builder.invoke_handler(tauri::generate_handler![read_settings, run_probe, quit]);
    builder
        .setup(|app| {
            let lab_enabled =
                cfg!(debug_assertions) && std::env::args().any(|a| a == "--overlay-lab");
            let filename = if lab_enabled {
                "overlay-lab-settings.json"
            } else {
                "settings.json"
            };
            let store = Store::open(app.path().app_config_dir()?.join(filename))
                .map_err(std::io::Error::other)?;
            let topmost = store.current.always_on_top;
            app.manage(Mutex::new(store));
            app.manage(crate::overlay::OverlayState::default());
            let window = app
                .get_webview_window("notch")
                .ok_or("Missing notch window")?;
            window.set_always_on_top(topmost)?;
            crate::overlay::place(&window).map_err(std::io::Error::other)?;
            let menu = crate::overlay::menu(app.handle())?;
            app.manage(menu.clone());
            let mut rgba = Vec::with_capacity(16 * 16 * 4);
            for _ in 0..16 * 16 {
                rgba.extend([184, 231, 218, 255]);
            }
            tauri::tray::TrayIconBuilder::new()
                .tooltip("LKOS — Foundation")
                .icon(tauri::image::Image::new_owned(rgba, 16, 16))
                .menu(&menu)
                .build(app)?;
            window.show()?;
            #[cfg(debug_assertions)]
            if lab_enabled {
                crate::lab::open(app.handle())?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
}

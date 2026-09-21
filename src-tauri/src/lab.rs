//! Debug-only manual spike, not a product feature or automation backdoor.
use crate::overlay::{self, OverlayState, SettingsState};
use serde::{Deserialize, Serialize};
use tauri::{Manager, WebviewWindow};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LabAction {
    PlaceTarget,
    Passive,
    Restore,
    Menu,
    DefaultPosition,
}

fn authorize(window: &WebviewWindow) -> Result<(), String> {
    if window.label() != "overlay-lab" || !std::env::args().any(|a| a == "--overlay-lab") {
        return Err("Overlay lab is disabled".into());
    }
    Ok(())
}

#[tauri::command]
pub async fn lab_action(window: WebviewWindow, action: LabAction) -> Result<(), String> {
    authorize(&window)?;
    let app = window.app_handle();
    let notch = app.get_webview_window("notch").ok_or("Missing notch")?;
    match action {
        LabAction::PlaceTarget => {
            let pos = window.inner_position().map_err(|e| e.to_string())?;
            let scale = window.scale_factor().map_err(|e| e.to_string())?;
            notch
                .set_position(tauri::PhysicalPosition::new(
                    pos.x + (32.0 * scale).round() as i32,
                    pos.y + (72.0 * scale).round() as i32,
                ))
                .map_err(|e| e.to_string())?;
        }
        LabAction::Passive => overlay::action(app, "passive")?,
        LabAction::Restore => overlay::action(app, "restore")?,
        LabAction::Menu => window
            .popup_menu_at(
                &app.state::<tauri::menu::Menu<tauri::Wry>>().inner().clone(),
                tauri::LogicalPosition::new(32.0, 330.0),
            )
            .map_err(|e| e.to_string())?,
        LabAction::DefaultPosition => overlay::place(&notch)?,
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    passive: bool,
    topmost: bool,
    notch_focused: bool,
    lab_focused: bool,
    scale: f64,
    position: tauri::PhysicalPosition<i32>,
    size: tauri::PhysicalSize<u32>,
    probe_count: u32,
    monitors: Vec<Monitor>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Monitor {
    scale: f64,
    work_position: tauri::PhysicalPosition<i32>,
    work_size: tauri::PhysicalSize<u32>,
}

#[tauri::command]
pub fn lab_snapshot(window: WebviewWindow) -> Result<Snapshot, String> {
    authorize(&window)?;
    let app = window.app_handle();
    let notch = app.get_webview_window("notch").ok_or("Missing notch")?;
    Ok(Snapshot {
        passive: *app
            .state::<OverlayState>()
            .passive
            .lock()
            .map_err(|e| e.to_string())?,
        topmost: notch.is_always_on_top().map_err(|e| e.to_string())?,
        notch_focused: notch.is_focused().map_err(|e| e.to_string())?,
        lab_focused: window.is_focused().map_err(|e| e.to_string())?,
        scale: notch.scale_factor().map_err(|e| e.to_string())?,
        position: notch.outer_position().map_err(|e| e.to_string())?,
        size: notch.outer_size().map_err(|e| e.to_string())?,
        probe_count: app
            .state::<SettingsState>()
            .lock()
            .map_err(|e| e.to_string())?
            .current
            .probe_count,
        monitors: notch
            .available_monitors()
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|m| Monitor {
                scale: m.scale_factor(),
                work_position: m.work_area().position,
                work_size: m.work_area().size,
            })
            .collect(),
    })
}

pub fn open(app: &tauri::AppHandle) -> tauri::Result<()> {
    tauri::WebviewWindowBuilder::new(
        app,
        "overlay-lab",
        tauri::WebviewUrl::App("index.html#overlay-lab".into()),
    )
    .title("LKOS — Overlay Lab")
    .inner_size(860.0, 680.0)
    .center()
    .build()?;
    Ok(())
}

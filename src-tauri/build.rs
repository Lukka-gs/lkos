fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&["read_settings", "run_probe", "quit"]),
    ))
    .expect("failed to build LKOS command permissions");
}

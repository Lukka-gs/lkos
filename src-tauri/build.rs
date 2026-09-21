fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "read_settings",
            "run_probe",
            "quit",
            "lab_action",
            "lab_snapshot",
        ]),
    ))
    .expect("failed to build LKOS command permissions");
}

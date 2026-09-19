#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod settings;
mod shell;

fn main() {
    if let Err(error) = shell::run() {
        eprintln!("LKOS startup failed: {error}");
        std::process::exit(1);
    }
}

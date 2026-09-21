#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(debug_assertions)]
mod lab;
mod overlay;
mod placement;
mod settings;
mod shell;

fn main() {
    if let Err(error) = shell::run() {
        eprintln!("LKOS startup failed: {error}");
        std::process::exit(1);
    }
}

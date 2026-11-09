// OmniDL app entry (Tauri 2). One codebase ships to Windows, macOS,
// Linux (desktop) and Android / iOS (Tauri Mobile, same UI).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use omnidl_core::{route, Backend};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Default)]
struct AppState {
    next_id: Mutex<u64>,
}

#[derive(Serialize)]
struct NewDownload {
    id: u64,
    backend: String,
    reason: String,
}

/// The UI calls this when a link is added: the core router decides the
/// backend, the engine (see crates/omnidl-engine) takes it from there
/// and streams progress events back to the window.
#[tauri::command]
fn add_download(state: tauri::State<'_, AppState>, url: String) -> NewDownload {
    let r = route(&url);
    let mut id = state.next_id.lock().unwrap();
    *id += 1;
    NewDownload {
        id: *id,
        backend: format!("{:?}", r.backend),
        reason: r.reason.to_string(),
    }
}

#[tauri::command]
fn pause_download(_id: u64) {}

#[tauri::command]
fn resume_download(_id: u64) {}

#[tauri::command]
fn remove_download(_id: u64) {}

#[tauri::command]
fn open_file(_id: u64) {}

fn main() {
    let _ = Backend::SegmentedHttp; // engine wiring lives in omnidl-engine
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            add_download,
            pause_download,
            resume_download,
            remove_download,
            open_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running OmniDL");
}

// MelderMap als eigenes Windows-Programm: ein Fenster mit der App, ohne Browser
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("MelderMap konnte nicht starten");
}

// MelderMap als eigenes Windows-Programm: ein Fenster mit der App, ohne Browser
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_updater::UpdaterExt;

// Gibt es eine neuere Version (latest.json auf der Web-Seite)? Antwort: Versionsnummer oder nichts
#[tauri::command]
async fn update_check(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let up = app.updater().map_err(|e| e.to_string())?;
    Ok(up.check().await.map_err(|e| e.to_string())?.map(|u| u.version))
}

// Update laden, Signatur prüfen und installieren; danach startet die neue Version
#[tauri::command]
async fn update_install(app: tauri::AppHandle) -> Result<(), String> {
    let up = app.updater().map_err(|e| e.to_string())?;
    if let Some(u) = up.check().await.map_err(|e| e.to_string())? {
        u.download_and_install(|_, _| {}, || {}).await.map_err(|e| e.to_string())?;
        app.restart();
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![update_check, update_install])
        .run(tauri::generate_context!())
        .expect("MelderMap konnte nicht starten");
}

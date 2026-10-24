// MelderMap als eigenes Windows-Programm: ein Fenster mit der App, ohne Browser
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_updater::UpdaterExt;

// Gibt es eine neuere Version (latest.json auf der Web-Seite)? Antwort: Versionsnummer oder nichts
#[tauri::command]
async fn update_check(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let up = app.updater().map_err(|e| e.to_string())?;
    Ok(up.check().await.map_err(|e| e.to_string())?.map(|u| u.version))
}

// Update laden und Signatur prüfen; danach beendet sich das Programm und der Installer läuft (startet MelderMap neu)
#[tauri::command]
async fn update_install(app: tauri::AppHandle) -> Result<(), String> {
    let up = app.updater().map_err(|e| e.to_string())?;
    let Some(u) = up.check().await.map_err(|e| e.to_string())? else { return Ok(()) };
    let bytes = u.download(|_, _| {}, || {}).await.map_err(|e| e.to_string())?;
    install_later(&bytes, &u.version).map_err(|e| e.to_string())?;
    app.exit(0);
    Ok(())
}

// Installer erst starten, wenn MelderMap schon beendet ist. Startet er sofort, findet er das laufende
// Programm und bricht mit „Failed to kill MelderMap“ ab.
#[cfg(windows)]
fn install_later(bytes: &[u8], version: &str) -> std::io::Result<()> {
    use std::os::windows::process::CommandExt;
    let path = std::env::temp_dir().join(format!("MelderMap_{version}_update.exe"));
    std::fs::write(&path, bytes)?;
    std::process::Command::new("cmd")
        .raw_arg(format!("/C \"ping -n 4 127.0.0.1 >nul & start \"\" \"{}\" /P /UPDATE /R\"", path.display()))
        .creation_flags(0x0800_0000) // kein Konsolenfenster
        .spawn()?;
    Ok(())
}
#[cfg(not(windows))]
fn install_later(_: &[u8], _: &str) -> std::io::Result<()> {
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![update_check, update_install])
        .run(tauri::generate_context!())
        .expect("MelderMap konnte nicht starten");
}

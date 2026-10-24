# MelderMap für Windows

Eigenes Programm (Tauri, nutzt die in Windows eingebaute WebView2), ohne Browserfenster.
Gebaut wird auf GitHub (Workflow „Windows-App“ im öffentlichen Repo), Ergebnis ist ein Installer
`MelderMap_<Version>_x64-setup.exe` unter Releases.

Neu bauen: `scripts/release-desktop.sh` (startet den Workflow; das Release heißt `v<Version>`).

## Updates

Das Programm fragt beim Start `https://marvinwildenau-hash.github.io/MelderMap/latest.json` ab. Gibt es eine neuere
Version, erscheint unten „Update installieren“: der Installer wird geladen, die Signatur geprüft und das Programm neu gestartet.
`scripts/release-desktop.sh` wartet nach dem Bau auf das Release, signiert den Installer mit dem Update-Schlüssel
(liegt nur in den Projektdateien unter `lizenz/`, nie im Repo) und legt `latest.json` ins öffentliche Repo.
Geht der Schlüssel verloren, können installierte Programme keine Updates mehr annehmen; dann einmal von Hand neu installieren.

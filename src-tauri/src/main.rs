// Empêche l'ouverture d'une console en plus de la fenêtre sur Windows (version finale)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        // fenêtre « Enregistrer sous… » et écriture du fichier choisi : rien d'autre
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("erreur au lancement d'ASCII Local");
}

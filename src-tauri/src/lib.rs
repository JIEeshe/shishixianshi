mod commands;
mod providers;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::get_overlay_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running shishixianshi");
}

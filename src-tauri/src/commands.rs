use crate::providers::{mock_overlay_snapshot, OverlaySnapshot};

#[tauri::command]
pub fn get_overlay_snapshot() -> OverlaySnapshot {
    mock_overlay_snapshot()
}

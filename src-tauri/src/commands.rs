use crate::providers::{OverlaySnapshot, build_overlay_snapshot};

#[tauri::command]
pub fn get_overlay_snapshot() -> OverlaySnapshot {
    build_overlay_snapshot()
}

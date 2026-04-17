mod claude;
mod codex;
mod types;

pub use types::{OverlaySnapshot, ProviderSnapshot};

pub fn build_overlay_snapshot() -> OverlaySnapshot {
    OverlaySnapshot {
        title: "quota hud".to_string(),
        mode: "codex live / claude auth".to_string(),
        updated_at: "30s refresh".to_string(),
        providers: vec![claude::claude_snapshot(), codex::codex_snapshot()],
    }
}

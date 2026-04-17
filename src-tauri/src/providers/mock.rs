use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::{OverlaySnapshot, ProviderSnapshot};

fn now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

fn hhmm(total_minutes: u64) -> String {
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    format!("{hours:02}:{minutes:02}")
}

fn status_for(remaining: u8) -> &'static str {
    if remaining <= 20 {
        "critical"
    } else if remaining <= 45 {
        "warn"
    } else {
        "ok"
    }
}

pub fn mock_overlay_snapshot() -> OverlaySnapshot {
    let tick = now_seconds();
    let claude_remaining = 72u8.saturating_sub((tick % 11) as u8);
    let codex_remaining = 58u8.saturating_sub((tick % 7) as u8);

    OverlaySnapshot {
        title: "quota hud".to_string(),
        mode: "tauri mock".to_string(),
        updated_at: format!("t+{}", tick % 100_000),
        providers: vec![
            ProviderSnapshot {
                id: "claude".to_string(),
                label: "claude".to_string(),
                remaining_label: format!("{claude_remaining}%"),
                remaining_percent: claude_remaining,
                window_label: "5h 窗口".to_string(),
                reset_in_label: hhmm(102),
                detail: "剩余会话额度".to_string(),
                status: status_for(claude_remaining).to_string(),
            },
            ProviderSnapshot {
                id: "codex".to_string(),
                label: "codex".to_string(),
                remaining_label: format!("{codex_remaining}%"),
                remaining_percent: codex_remaining,
                window_label: "周额度".to_string(),
                reset_in_label: "2d 04h".to_string(),
                detail: "剩余本周额度".to_string(),
                status: status_for(codex_remaining).to_string(),
            },
        ],
    }
}

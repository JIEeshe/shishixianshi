use std::collections::HashMap;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Deserialize;
use serde_json::{Value, json};

use super::ProviderSnapshot;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetAccountRateLimitsResponse {
    rate_limits: RateLimitSnapshot,
    rate_limits_by_limit_id: Option<HashMap<String, RateLimitSnapshot>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RateLimitSnapshot {
    limit_id: Option<String>,
    limit_name: Option<String>,
    primary: Option<RateLimitWindow>,
    secondary: Option<RateLimitWindow>,
    credits: Option<CreditsSnapshot>,
    plan_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RateLimitWindow {
    used_percent: u8,
    window_duration_mins: Option<u64>,
    resets_at: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreditsSnapshot {
    balance: Option<String>,
    has_credits: bool,
    unlimited: bool,
}

#[derive(Debug, Deserialize)]
struct RpcResponseEnvelope {
    id: Option<u64>,
    result: Option<Value>,
    error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    message: String,
}

fn resolve_codex_path() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        let candidates = [
            format!("{home}/.npm-global/bin/codex"),
            format!("{home}/.local/bin/codex"),
        ];

        for candidate in candidates {
            let path = PathBuf::from(candidate);
            if path.exists() {
                return path;
            }
        }
    }

    PathBuf::from("codex")
}

fn now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

fn remaining_percent(window: &RateLimitWindow) -> u8 {
    100u8.saturating_sub(window.used_percent.min(100))
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

fn format_window_label(window: &RateLimitWindow, limit_name: Option<&str>) -> String {
    if let Some(minutes) = window.window_duration_mins {
        if minutes >= 60 * 24 {
            let days = minutes / (60 * 24);
            return format!("{days}d 窗口");
        }

        if minutes >= 60 {
            return format!("{}h 窗口", minutes / 60);
        }

        return format!("{minutes}m 窗口");
    }

    limit_name.unwrap_or("额度窗口").to_string()
}

fn format_reset_label(resets_at: Option<u64>) -> String {
    let Some(resets_at) = resets_at else {
        return "--".to_string();
    };

    let now = now_seconds();
    let remaining = resets_at.saturating_sub(now);

    if remaining == 0 {
        return "soon".to_string();
    }

    if remaining >= 86_400 {
        let days = remaining / 86_400;
        let hours = (remaining % 86_400) / 3_600;
        return format!("{days}d {hours:02}h");
    }

    let hours = remaining / 3_600;
    let minutes = (remaining % 3_600) / 60;
    format!("{hours:02}:{minutes:02}")
}

fn short_error_detail(message: &str) -> String {
    if message.contains("Could not resolve host") {
        return "网络不可用，无法拉取额度".to_string();
    }

    if message.contains("usage") {
        return "无法读取 ChatGPT 配额".to_string();
    }

    if message.contains("auth") || message.contains("login") {
        return "Codex 未登录".to_string();
    }

    message.chars().take(44).collect()
}

fn read_rate_limits() -> Result<GetAccountRateLimitsResponse, String> {
    let mut child = Command::new(resolve_codex_path())
        .args(["app-server", "--listen", "stdio://"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("failed to start codex app-server: {error}"))?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "failed to open codex stdin".to_string())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "failed to open codex stdout".to_string())?;

    let (sender, receiver) = mpsc::channel();

    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let Ok(line) = line else {
                continue;
            };

            let Ok(envelope) = serde_json::from_str::<RpcResponseEnvelope>(&line) else {
                continue;
            };

            if envelope.id != Some(2) {
                continue;
            }

            if let Some(error) = envelope.error {
                let _ = sender.send(Err(error.message));
                return;
            }

            if let Some(result) = envelope.result {
                let parsed = serde_json::from_value::<GetAccountRateLimitsResponse>(result)
                    .map_err(|error| format!("failed to decode codex rate limits: {error}"));
                let _ = sender.send(parsed);
                return;
            }
        }

        let _ = sender.send(Err("codex app-server returned no rate-limit response".to_string()));
    });

    let initialize = json!({
        "method": "initialize",
        "id": 1,
        "params": {
            "clientInfo": {
                "name": "quota-hud",
                "title": "Quota HUD",
                "version": "0.1.0"
            },
            "capabilities": {
                "experimentalApi": true
            }
        }
    });
    let read_request = json!({
        "method": "account/rateLimits/read",
        "id": 2
    });

    writeln!(stdin, "{initialize}")
        .map_err(|error| format!("failed to write initialize request: {error}"))?;
    writeln!(stdin, "{read_request}")
        .map_err(|error| format!("failed to write rate-limit request: {error}"))?;
    drop(stdin);

    let result = receiver
        .recv_timeout(Duration::from_secs(6))
        .map_err(|_| "timed out waiting for codex rate limits".to_string())?;

    let _ = child.kill();
    let _ = child.wait();

    result
}

pub fn codex_snapshot() -> ProviderSnapshot {
    let response = match read_rate_limits() {
        Ok(response) => response,
        Err(message) => {
            return ProviderSnapshot {
                id: "codex".to_string(),
                label: "codex".to_string(),
                remaining_label: "ERR".to_string(),
                remaining_percent: 0,
                window_label: "app-server".to_string(),
                reset_in_label: "--".to_string(),
                detail: short_error_detail(&message),
                status: "error".to_string(),
            };
        }
    };

    let snapshot = response
        .rate_limits_by_limit_id
        .as_ref()
        .and_then(|limits| limits.get("codex").cloned())
        .unwrap_or(response.rate_limits);

    let primary = snapshot.primary.clone().or(snapshot.secondary.clone());

    let Some(primary) = primary else {
        return ProviderSnapshot {
            id: "codex".to_string(),
            label: "codex".to_string(),
            remaining_label: "--".to_string(),
            remaining_percent: 0,
            window_label: "app-server".to_string(),
            reset_in_label: "--".to_string(),
            detail: "未返回额度窗口".to_string(),
            status: "error".to_string(),
        };
    };

    let remaining = remaining_percent(&primary);
    let mut detail_parts = Vec::new();

    if let Some(plan_type) = snapshot.plan_type {
        detail_parts.push(plan_type);
    }

    if let Some(secondary) = snapshot.secondary {
        detail_parts.push(format!("次级 {}%", remaining_percent(&secondary)));
    }

    if let Some(credits) = snapshot.credits {
        if credits.unlimited {
            detail_parts.push("unlimited".to_string());
        } else if credits.has_credits {
            if let Some(balance) = credits.balance {
                detail_parts.push(format!("credits {balance}"));
            }
        }
    }

    if let Some(limit_id) = snapshot.limit_id {
        detail_parts.push(limit_id);
    }

    ProviderSnapshot {
        id: "codex".to_string(),
        label: "codex".to_string(),
        remaining_label: format!("{remaining}%"),
        remaining_percent: remaining,
        window_label: format_window_label(&primary, snapshot.limit_name.as_deref()),
        reset_in_label: format_reset_label(primary.resets_at),
        detail: if detail_parts.is_empty() {
            "实时额度".to_string()
        } else {
            detail_parts.join(" / ")
        },
        status: status_for(remaining).to_string(),
    }
}

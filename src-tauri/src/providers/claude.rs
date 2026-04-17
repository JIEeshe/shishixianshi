use std::env;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

use super::ProviderSnapshot;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeAuthStatus {
    logged_in: bool,
    auth_method: String,
    api_provider: String,
}

fn resolve_claude_path() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        let candidates = [
            format!("{home}/.local/bin/claude"),
            format!("{home}/.npm-global/bin/claude"),
        ];

        for candidate in candidates {
            let path = PathBuf::from(candidate);
            if path.exists() {
                return path;
            }
        }
    }

    PathBuf::from("claude")
}

pub fn claude_snapshot() -> ProviderSnapshot {
    let output = match Command::new(resolve_claude_path())
        .args(["auth", "status"])
        .output()
    {
        Ok(output) => output,
        Err(_) => {
            return ProviderSnapshot {
                id: "claude".to_string(),
                label: "claude".to_string(),
                remaining_label: "--".to_string(),
                remaining_percent: 0,
                window_label: "auth".to_string(),
                reset_in_label: "--".to_string(),
                detail: "Claude CLI 未安装".to_string(),
                status: "error".to_string(),
            };
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let auth = serde_json::from_str::<ClaudeAuthStatus>(&stdout);

    match auth {
        Ok(status) if status.logged_in => ProviderSnapshot {
            id: "claude".to_string(),
            label: "claude".to_string(),
            remaining_label: "--".to_string(),
            remaining_percent: 0,
            window_label: "auth".to_string(),
            reset_in_label: "--".to_string(),
            detail: format!(
                "已登录 ({}/{})，额度源待接入",
                status.auth_method, status.api_provider
            ),
            status: "warn".to_string(),
        },
        Ok(_) => ProviderSnapshot {
            id: "claude".to_string(),
            label: "claude".to_string(),
            remaining_label: "--".to_string(),
            remaining_percent: 0,
            window_label: "auth".to_string(),
            reset_in_label: "--".to_string(),
            detail: "Claude CLI 未登录".to_string(),
            status: "error".to_string(),
        },
        Err(_) => ProviderSnapshot {
            id: "claude".to_string(),
            label: "claude".to_string(),
            remaining_label: "--".to_string(),
            remaining_percent: 0,
            window_label: "auth".to_string(),
            reset_in_label: "--".to_string(),
            detail: "Claude 认证状态解析失败".to_string(),
            status: "error".to_string(),
        },
    }
}

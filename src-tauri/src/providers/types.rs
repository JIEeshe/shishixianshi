use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSnapshot {
    pub id: String,
    pub label: String,
    pub remaining_label: String,
    pub remaining_percent: u8,
    pub window_label: String,
    pub reset_in_label: String,
    pub detail: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlaySnapshot {
    pub title: String,
    pub mode: String,
    pub updated_at: String,
    pub providers: Vec<ProviderSnapshot>,
}

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct ModLog {
    pub enabled: bool,
    pub interval: i64,
    pub notify_bans: bool,
    pub notify_federated_bans: bool,
    pub notify_removals: bool,
    pub allowlist_federated_actions: Vec<String>,
}

impl Default for ModLog {
    fn default() -> Self {
        ModLog {
            enabled: false,
            interval: 60,
            notify_bans: false,
            notify_federated_bans: false,
            notify_removals: false,
            allowlist_federated_actions: vec![],
        }
    }
}

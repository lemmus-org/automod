use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct PrivateMessage {
    pub enabled: bool,
    pub interval: i64,
    pub prune_messages: bool,
    pub forward_messages: bool,
    pub allow_message_commands: bool,
    pub audit_message_commands: bool,
}

impl Default for PrivateMessage {
    fn default() -> Self {
        PrivateMessage {
            enabled: false,
            interval: 60,
            prune_messages: false,
            forward_messages: false,
            allow_message_commands: false,
            audit_message_commands: false,
        }
    }
}

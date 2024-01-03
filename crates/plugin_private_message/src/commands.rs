use std::fmt::{Display, Formatter};

const PREFIX: char = '!';
const PURGE_USER: &str = "purge_user";
const SITE_BAN: &str = "site_ban";

pub enum Commands {
    SiteBan(String, String),
    PurgeUser(String, String),
}

impl Commands {
    pub fn parse(value: &str) -> Option<Self> {
        if !value.starts_with(PREFIX) {
            return None;
        }

        let parts = value.splitn(3, ' ').collect::<Vec<&str>>();
        match &parts[0][1..] {
            PURGE_USER => {
                if parts.len() < 3 {
                    return None;
                }

                let username = parts[1].to_string();
                let reason = parts[2].to_string();

                Some(Commands::PurgeUser(username, reason))
            }
            SITE_BAN => {
                if parts.len() < 3 {
                    return None;
                }

                let username = parts[1].to_string();
                let reason = parts[2].to_string();

                Some(Commands::SiteBan(username, reason))
            }
            _ => None,
        }
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::PurgeUser(username, reason) => {
                write!(
                    f,
                    "* command = `{}`\r\n\
                     * user = {}\r\n\
                     * reason = `{}`",
                    PURGE_USER, username, reason
                )
            }
            Commands::SiteBan(username, reason) => {
                write!(
                    f,
                    "* command = `{}`\r\n\
                     * user = {}\r\n\
                     * reason = `{}`",
                    SITE_BAN, username, reason
                )
            }
        }
    }
}

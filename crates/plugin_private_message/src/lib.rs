use crate::commands::Commands;
use chrono::{DateTime, Duration, Utc};
use lemmy_client::model::Person;
use lemmy_client::person::{person_ban, person_get, person_purge, PersonRef};
use lemmy_client::private_message::{
    private_message_create, private_message_delete, private_message_list, private_message_read,
};
use lemmy_client::site::site_admins_get;
use lemmy_client::{model, Client, ClientError};
use plugin_common::notify_admins;

mod commands;
pub mod config;

pub struct PrivateMessage {
    config: config::PrivateMessage,
    last_run: DateTime<Utc>,
}

impl PrivateMessage {
    pub fn new(config: config::PrivateMessage) -> Self {
        PrivateMessage {
            config,
            last_run: Utc::now(),
        }
    }

    pub async fn run(&mut self, client: &Client) {
        // Ensure plugin is enabled
        if !self.config.enabled {
            return;
        }

        let now = Utc::now();
        let since = self.last_run;
        let elapsed = now - since;

        // Validate plugin is scheduled to run
        if elapsed.num_seconds() < self.config.interval {
            return;
        }
        self.last_run = now;

        println!("Checking private messages...");

        if self.config.prune_messages {
            // Prune private messages
            prune_messages(client, now).await;
        }

        if self.config.allow_message_commands || self.config.forward_messages {
            // Check private messages
            self.check_messages(client).await;
        }

        println!("Finished checking private messages!");
    }

    async fn check_messages(&self, client: &Client) {
        // Get any unread message
        let unread_messages = match private_message_list(client, true).await {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        // Get list of local admins
        let admins = match site_admins_get(client).await {
            Ok(admins) => admins,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        // Check each unread message
        for message in unread_messages {
            // Skip our own messages
            if message.sender_id == client.user_id() {
                continue;
            }

            // Mark message as read
            if let Err(err) = private_message_read(client, message.id).await {
                println!("{}", err);
                continue;
            }

            // Fetch sender details
            let person = match person_get(client, PersonRef::Id(message.sender_id)).await {
                Ok(value) => value,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            let is_local = person.is_local;
            let is_admin = admins.iter().any(|admin| admin.id == person.id);

            // Verify sender is a local admin
            // NOTE: This could be extended to support limited commands for moderators
            if is_local && is_admin {
                if self.config.allow_message_commands {
                    // Perform command contained within the message
                    perform_message_commands(
                        client,
                        &admins,
                        &person,
                        &message,
                        self.config.audit_message_commands,
                    )
                    .await;
                }
            } else if self.config.forward_messages {
                // Forward message to admins
                let body = format!(
                    "* user = {}\r\n\
                    * message = `{}`",
                    person, message
                );
                notify_admins(client, &admins, body).await;
            }
        }
    }
}

async fn prune_messages(client: &Client, now: DateTime<Utc>) {
    // Get any read messages
    let read_messages = match private_message_list(client, false).await {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // Delete any read messages older than threshold
    let stale_threshold = now - Duration::days(7);
    for message in read_messages {
        // Skip other's messages
        if message.sender_id != client.user_id() {
            continue;
        }

        if message.created <= stale_threshold {
            if let Err(err) = private_message_delete(client, message.id).await {
                println!("{}", err);
                continue;
            }
        }
    }
}

async fn perform_message_commands(
    client: &Client,
    admins: &Vec<Person>,
    sender: &Person,
    message: &model::PrivateMessage,
    auditlog: bool,
) {
    // Verify message is a command
    let content = message.content.trim();
    let command = match Commands::parse(content) {
        None => {
            let body = format!("invalid or unsupported command: `{}`", content);
            if let Err(err) = private_message_create(client, message.sender_id, body).await {
                println!("{}", err);
            }
            return;
        }
        Some(command) => {
            println!("Received command: {}", command);
            command
        }
    };

    // Execute command
    let action = command.to_string();
    match perform_command(client, command, admins).await {
        Ok(_) => {
            if auditlog {
                let body = format!(
                    "`{}` performed a message command:\r\n{}",
                    sender.name, action
                );
                notify_admins(client, admins, body.clone()).await;
            }
        }
        // Let the sender know the command failed
        Err(err) => {
            let body = format!("command failed: `{}`", err);
            if let Err(err) = private_message_create(client, message.sender_id, body).await {
                println!("{}", err);
            }
        }
    };
}

async fn perform_command(
    client: &Client,
    command: Commands,
    admins: &[Person],
) -> Result<(), ClientError> {
    match command {
        Commands::PurgeUser(username, reason) => {
            // Get target user
            let person = match person_get(client, PersonRef::Username(username)).await {
                Ok(value) => value,
                Err(err) => {
                    println!("{}", err);
                    return Err(err);
                }
            };

            // Verify user is not a local admin
            if admins.iter().any(|admin| admin.id == person.id) || person.id == client.user_id() {
                return Ok(());
            }

            // Perform user purge
            if let Err(err) = person_purge(client, person.id, Some(reason)).await {
                println!("{}", err);
                return Err(err);
            }

            Ok(())
        }
        Commands::SiteBanRemove(username, reason) => {
            perform_site_ban(client, admins, username, reason, true).await
        }
        Commands::SiteBan(username, reason) => {
            perform_site_ban(client, admins, username, reason, false).await
        }
    }
}

async fn perform_site_ban(
    client: &Client,
    admins: &[Person],
    username: String,
    reason: String,
    remove_content: bool,
) -> Result<(), ClientError> {
    // Get target user
    let person = match person_get(client, PersonRef::Username(username)).await {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return Err(err);
        }
    };

    // Verify user is not a local admin
    if admins.iter().any(|admin| admin.id == person.id) || person.id == client.user_id() {
        return Ok(());
    }

    // Perform user ban
    if let Err(err) = person_ban(
        client,
        person.id,
        true,
        Some(remove_content),
        Some(reason),
        None,
    )
    .await
    {
        println!("{}", err);
        return Err(err);
    }

    Ok(())
}

use chrono::{DateTime, Utc};
use lemmy_client::model::{ModlogBan, ModlogRemoval, Person};
use lemmy_client::modlog::modlog_local_get;
use lemmy_client::person::person_ban;
use lemmy_client::site::site_admins_get;
use lemmy_client::Client;
use plugin_common::notify_admins;

pub mod config;

pub struct ModLog {
    config: config::ModLog,
    last_run: DateTime<Utc>,
}

impl ModLog {
    pub fn new(config: config::ModLog) -> Self {
        ModLog {
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

        println!("Checking modlog...");

        // Get all modlog actions against local users since last run
        match modlog_local_get(client, since).await {
            Ok(actions) => {
                // Get list of local admins
                let admins = match site_admins_get(client).await {
                    Ok(admins) => admins,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                let notify_bans = self.config.notify_bans;
                let notify_federated_bans = self.config.notify_federated_bans;
                let allowlist = &self.config.allowlist_federated_actions;

                if notify_bans || !allowlist.is_empty() {
                    for action in actions.bans {
                        if notify_bans {
                            // Notify admins on any ban actions
                            notify_admins(client, &admins, action.to_string()).await;
                        }

                        if !allowlist.is_empty() {
                            // Federate bans from allowed instances
                            federate_ban_action(
                                client,
                                &admins,
                                allowlist,
                                action,
                                notify_federated_bans,
                            )
                            .await;
                        }
                    }
                }

                if self.config.notify_removals {
                    // Notify admins on any removal actions
                    for action in actions.removals {
                        match action {
                            ModlogRemoval::Comment(comment) => {
                                notify_admins(client, &admins, comment.to_string()).await;
                            }
                            ModlogRemoval::Post(post) => {
                                notify_admins(client, &admins, post.to_string()).await;
                            }
                        }
                    }
                }
            }
            Err(err) => println!("{}", err),
        }

        println!("Finished checking modlog!");
    }
}

async fn federate_ban_action(
    client: &Client,
    admins: &Vec<Person>,
    allowlist: &[String],
    ban: ModlogBan,
    notify: bool,
) {
    let action = ban.to_string();
    if let ModlogBan::Site {
        moderator,
        user,
        is_banned,
        expires,
        ..
    } = ban
    {
        // Only act on local, non-admin users
        if !user.is_local
            || admins.iter().any(|admin| admin.id == user.id)
            || user.id == client.user_id()
        {
            return;
        }

        // Verify that the moderator's instance has been allowed
        if allowlist.contains(&moderator.instance) {
            // Perform ban locally
            let reason = format!("Federated ban from {}", moderator.instance);
            if let Err(err) =
                person_ban(client, user.id, is_banned, None, Some(reason), expires).await
            {
                println!("{}", err);
                return;
            }

            if notify {
                // Notify admins of federated action
                let message = format!("Federated ban:\r\n{}", action);
                notify_admins(client, admins, message).await;
            }
        }
    }
}

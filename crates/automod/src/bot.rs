use crate::config::Plugins;
use lemmy_client::Client;
use plugin_mod_log::ModLog;
use plugin_private_message::PrivateMessage;
use std::time::Duration;

pub struct Bot {
    mod_log: ModLog,
    private_message: PrivateMessage,
}

impl Bot {
    pub fn new(config: Plugins) -> Self {
        Bot {
            mod_log: ModLog::new(config.mod_log),
            private_message: PrivateMessage::new(config.private_message),
        }
    }

    pub async fn run(&mut self, client: Client) {
        println!("Starting lemmy automod!");

        loop {
            // Invoke each plugin
            // TODO: Run in parallel
            self.mod_log.run(&client).await;
            self.private_message.run(&client).await;

            // Await next iteration
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}

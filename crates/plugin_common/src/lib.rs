use lemmy_client::model::Person;
use lemmy_client::private_message::private_message_create;
use lemmy_client::Client;
use tracing::error;

pub async fn notify_admins(client: &Client, admins: &Vec<Person>, message: String) {
    for admin in admins {
        // Send private message
        if let Err(err) = private_message_create(client, admin.id, message.clone()).await {
            error!("{}", err);
            continue;
        }
    }
}

use lemmy_client::model::Person;
use lemmy_client::person::{person_get, PersonRef};
use lemmy_client::private_message::private_message_create;
use lemmy_client::Client;

pub async fn notify_admins(client: &Client, admins: &Vec<Person>, message: String) {
    for admin in admins {
        // Get reference to admin user
        match person_get(client, PersonRef::Username(admin.name.clone())).await {
            Ok(person) => {
                // Send private message
                if let Err(err) = private_message_create(client, person.id, message.clone()).await {
                    println!("{}", err);
                    continue;
                }
            }
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
}

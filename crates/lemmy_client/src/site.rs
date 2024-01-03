use crate::endpoints::SITE;
use crate::model::Person;
use crate::{Client, ClientError};
use lemmy_api_common::site::GetSiteResponse;
use reqwest::StatusCode;

pub async fn site_admins_get(client: &Client) -> Result<Vec<Person>, ClientError> {
    // Create and perform request
    let path = SITE;
    let result = client.get(path, false).send().await;
    if let Err(err) = result {
        return Err(ClientError::new(path, err.to_string()));
    }

    // Validate response status
    let response = result.ok().unwrap();
    match response.status() {
        StatusCode::OK => {
            // Parse response body
            let body = response.json::<GetSiteResponse>().await;
            if let Err(err) = body {
                return Err(ClientError::new(path, err.to_string()));
            }
            let site = body.ok().unwrap();

            let admins = site
                .admins
                .iter()
                .filter(|view| view.person.id.0 != client.user_id())
                .map(|view| Person::from(view.person.clone()))
                .collect::<Vec<Person>>();

            Ok(admins)
        }
        status => Err(ClientError::new(path, status.to_string())),
    }
}

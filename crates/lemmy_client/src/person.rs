use crate::endpoints::{ADMIN_PURGE_USER, USER, USER_BAN};
use crate::model::Person;
use crate::{Client, ClientError};
use chrono::{DateTime, Utc};
use lemmy_api_common::lemmy_db_schema::newtypes::PersonId;
use lemmy_api_common::person::{BanPerson, GetPersonDetails, GetPersonDetailsResponse};
use lemmy_api_common::site::PurgePerson;
use reqwest::StatusCode;

pub enum PersonRef {
    Id(i32),
    Username(String),
}

pub async fn person_get(client: &Client, person_ref: PersonRef) -> Result<Person, ClientError> {
    // Create request
    let path = USER;
    let mut params = GetPersonDetails::default();
    match person_ref {
        PersonRef::Id(id) => {
            params.person_id = Some(PersonId(id));
        }
        PersonRef::Username(username) => {
            params.username = Some(username);
        }
    }

    // Perform request
    let result = client.get(path, false).query(&params).send().await;
    if let Err(err) = result {
        return Err(ClientError::new(path, err.to_string()));
    }

    // Validate response status
    let response = result.ok().unwrap();
    match response.status() {
        StatusCode::OK => {
            // Parse response body
            let body = response.json::<GetPersonDetailsResponse>().await;
            if let Err(err) = body {
                return Err(ClientError::new(path, err.to_string()));
            }
            let person = body.ok().unwrap();
            let user = Person::from(person.person_view.person);

            Ok(user)
        }
        status => Err(ClientError::new(path, status.to_string())),
    }
}

pub async fn person_ban(
    client: &Client,
    person_id: i32,
    ban: bool,
    remove_content: Option<bool>,
    reason: Option<String>,
    expires: Option<DateTime<Utc>>,
) -> Result<(), ClientError> {
    // Create request
    let path = USER_BAN;
    let params = BanPerson {
        person_id: PersonId(person_id),
        ban,
        remove_data: remove_content,
        reason,
        expires: expires.map(|exp| exp.timestamp()),
    };

    // Perform request
    let result = client.post(path, true).json(&params).send().await;
    if let Err(err) = result {
        return Err(ClientError::new(path, err.to_string()));
    }

    // Validate response status
    let response = result.ok().unwrap();
    match response.status() {
        StatusCode::OK => Ok(()),
        status => Err(ClientError::new(path, status.to_string())),
    }
}

pub async fn person_purge(
    client: &Client,
    person_id: i32,
    reason: Option<String>,
) -> Result<(), ClientError> {
    // Create request
    let path = ADMIN_PURGE_USER;
    let params = PurgePerson {
        person_id: PersonId(person_id),
        reason,
    };

    // Perform request
    let result = client.post(path, true).json(&params).send().await;
    if let Err(err) = result {
        return Err(ClientError::new(path, err.to_string()));
    }

    // Validate response status
    let response = result.ok().unwrap();
    match response.status() {
        StatusCode::OK => Ok(()),
        status => Err(ClientError::new(path, status.to_string())),
    }
}

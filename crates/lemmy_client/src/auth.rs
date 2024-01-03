use crate::endpoints::LOGIN;
use crate::model::Authentication;
use crate::person::{person_get, PersonRef};
use crate::{Client, ClientError};
use lemmy_api_common::person::{Login, LoginResponse};
use lemmy_api_common::sensitive::Sensitive;
use reqwest::StatusCode;

pub async fn login(
    client: &Client,
    username: String,
    password: String,
) -> Result<Authentication, ClientError> {
    // Create request
    let path = LOGIN;
    let body = Login {
        username_or_email: Sensitive::from(username.clone()),
        password: Sensitive::from(password),
        totp_2fa_token: None,
    };

    // Perform request
    let result = client.post(path, false).json(&body).send().await;
    if let Err(err) = result {
        return Err(ClientError::new(path, err.to_string()));
    }

    // Validate response status
    let response = result.ok().unwrap();
    match response.status() {
        StatusCode::CREATED | StatusCode::OK => {
            // Parse response body
            let body = response.json::<LoginResponse>().await;
            if let Err(err) = body {
                return Err(ClientError::new(path, err.to_string()));
            }
            let user = body.ok().unwrap();

            // Validate auth token
            if user.jwt.is_none() {
                return Err(ClientError::new(path, "empty jwt".to_string()));
            }

            // Set client auth token
            let jwt = user.jwt.unwrap().into_inner();

            // Set user id
            let user_id = match person_get(client, PersonRef::Username(username)).await {
                Ok(person) => person.id,
                Err(err) => {
                    return Err(err);
                }
            };

            Ok(Authentication::new(jwt, user_id))
        }
        status => Err(ClientError::new(path, status.to_string())),
    }
}

use crate::endpoints::POST_REPORT;
use crate::{Client, ClientError};
use lemmy_api_common::lemmy_db_schema::newtypes::PostId;
use lemmy_api_common::post::CreatePostReport;
use reqwest::StatusCode;

pub async fn post_report(client: &Client, post_id: i32, reason: String) -> Result<(), ClientError> {
    // Create and perform request
    let path = POST_REPORT;
    let body = CreatePostReport {
        post_id: PostId(post_id),
        reason,
    };
    let result = client.post(path, true).json(&body).send().await;
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

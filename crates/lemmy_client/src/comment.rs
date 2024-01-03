use crate::endpoints::COMMENT_REPORT;
use crate::{Client, ClientError};
use lemmy_api_common::comment::CreateCommentReport;
use lemmy_api_common::lemmy_db_schema::newtypes::CommentId;
use reqwest::StatusCode;

pub async fn comment_report(
    client: &Client,
    comment_id: i32,
    reason: String,
) -> Result<(), ClientError> {
    // Create and perform request
    let path = COMMENT_REPORT;
    let body = CreateCommentReport {
        comment_id: CommentId(comment_id),
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

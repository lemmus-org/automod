use crate::endpoints::{
    PRIVATE_MESSAGE, PRIVATE_MESSAGE_DELETE, PRIVATE_MESSAGE_LIST, PRIVATE_MESSAGE_READ,
};
use crate::model::PrivateMessage;
use crate::{Client, ClientError};
use lemmy_api_common::lemmy_db_schema::newtypes::{PersonId, PrivateMessageId};
use lemmy_api_common::private_message::{
    CreatePrivateMessage, DeletePrivateMessage, GetPrivateMessages, MarkPrivateMessageAsRead,
    PrivateMessageResponse, PrivateMessagesResponse,
};
use reqwest::StatusCode;

pub async fn private_message_create(
    client: &Client,
    recipient_id: i32,
    message: String,
) -> Result<PrivateMessage, ClientError> {
    // Create and perform request
    let path = PRIVATE_MESSAGE;
    let body = CreatePrivateMessage {
        content: message,
        recipient_id: PersonId(recipient_id),
    };
    let result = client.post(path, true).json(&body).send().await;
    if let Err(err) = result {
        return Err(ClientError::new(path, err.to_string()));
    }

    // Validate response status
    let response = result.ok().unwrap();
    match response.status() {
        StatusCode::OK => {
            // Parse response body
            let body = response.json::<PrivateMessageResponse>().await;
            if let Err(err) = body {
                return Err(ClientError::new(path, err.to_string()));
            }
            let private_message = body.ok().unwrap();
            let message =
                PrivateMessage::from(private_message.private_message_view.private_message);

            Ok(message)
        }
        status => Err(ClientError::new(path, status.to_string())),
    }
}

pub async fn private_message_delete(
    client: &Client,
    message_id: PrivateMessageId,
) -> Result<(), ClientError> {
    // Create and perform request
    let path = PRIVATE_MESSAGE_DELETE;
    let body = DeletePrivateMessage {
        private_message_id: message_id,
        deleted: true,
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

pub async fn private_message_list(
    client: &Client,
    unread: bool,
) -> Result<Vec<PrivateMessage>, ClientError> {
    // Create and perform request
    let path = PRIVATE_MESSAGE_LIST;
    let body = GetPrivateMessages {
        unread_only: Some(unread),
        limit: Some(50),
        ..Default::default()
    };
    let result = client.get(path, true).query(&body).send().await;
    if let Err(err) = result {
        return Err(ClientError::new(path, err.to_string()));
    }

    // Validate response status
    let response = result.ok().unwrap();
    match response.status() {
        StatusCode::OK => {
            // Parse response body
            let body = response.json::<PrivateMessagesResponse>().await;
            if let Err(err) = body {
                return Err(ClientError::new(path, err.to_string()));
            }
            let list = body.ok().unwrap();

            let messages = list
                .private_messages
                .iter()
                .map(|view| PrivateMessage::from(view.private_message.clone()))
                .collect::<Vec<PrivateMessage>>();

            Ok(messages)
        }
        status => Err(ClientError::new(path, status.to_string())),
    }
}

pub async fn private_message_read(
    client: &Client,
    message_id: PrivateMessageId,
) -> Result<(), ClientError> {
    // Create and perform request
    let path = PRIVATE_MESSAGE_READ;
    let body = MarkPrivateMessageAsRead {
        private_message_id: message_id,
        read: true,
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

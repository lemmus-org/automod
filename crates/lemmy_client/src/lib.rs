use crate::auth::login;
use crate::model::Authentication;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use std::fmt::{Display, Formatter};

pub mod auth;
pub mod comment;
pub mod endpoints;
pub mod model;
pub mod modlog;
pub mod person;
pub mod post;
pub mod private_message;
pub mod site;

pub struct ClientError {
    path: String,
    message: String,
}

impl ClientError {
    pub fn new(path: &str, message: String) -> Self {
        ClientError {
            path: path.to_string(),
            message,
        }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lemmy Client Error, path: '{}', error: '{}'",
            self.path, self.message
        )
    }
}

pub struct Client {
    http: ClientWithMiddleware,
    host: String,
    authentication: Authentication,
}

impl Client {
    pub async fn new(
        host: String,
        username: String,
        password: String,
    ) -> Result<Self, ClientError> {
        let mut client = Client {
            http: new_http_client(),
            host,
            authentication: Authentication::empty(),
        };

        // Authenticate
        match login(&client, username, password).await {
            Ok(auth) => {
                client.authentication = auth;
            }
            Err(err) => {
                return Err(err);
            }
        };

        Ok(client)
    }

    pub fn user_id(&self) -> i32 {
        self.authentication.user_id
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.host, path)
    }

    fn authorization(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.bearer_auth(&self.authentication.jwt)
    }

    fn get(&self, path: &str, authenticate: bool) -> RequestBuilder {
        let request = self.http.get(self.url(path));
        match authenticate {
            true => self.authorization(request),
            false => request,
        }
    }

    fn post(&self, path: &str, authenticate: bool) -> RequestBuilder {
        let request = self.http.post(self.url(path));
        match authenticate {
            true => self.authorization(request),
            false => request,
        }
    }
}

fn new_http_client() -> ClientWithMiddleware {
    ClientBuilder::new(reqwest::Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: HttpCacheOptions::default(),
        }))
        .build()
}

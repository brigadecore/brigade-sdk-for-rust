use crate::error::BrigadeError;
use reqwest::{IntoUrl, Method, RequestBuilder};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub allow_insecure_connections: bool,
}

impl ClientConfig {
    pub fn new() -> Self {
        Self {
            allow_insecure_connections: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub config: ClientConfig,
    pub address: String,
    pub token: Option<String>,
    rest: reqwest::Client,
}

impl Client {
    pub fn new(
        address: String,
        config: ClientConfig,
        token: Option<String>,
    ) -> Result<Self, BrigadeError> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(config.allow_insecure_connections)
            .build()?;
        Ok(Client {
            config,
            address,
            token,
            rest: client,
        })
    }

    pub fn req<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        let req = self.rest.request(method, url);
        match self.token.as_ref() {
            Some(t) => req.bearer_auth(t),
            None => req,
        }
    }
}

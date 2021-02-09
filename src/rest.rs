use crate::error::BrigadeError;
use hyper::Method;
use reqwest::Response;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub allow_insecure_connections: bool,
}

impl ClientConfig {
    pub fn default() -> Self {
        Self {
            allow_insecure_connections: false,
        }
    }
}

pub struct Client {
    pub config: ClientConfig,
    pub address: String,
    pub token: Option<String>,
    pub rest: reqwest::Client,
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
            token: token,
            rest: client,
        })
    }

    pub async fn req(&self, url: String, method: Method) -> Result<Response, BrigadeError> {
        let token = self.token.clone().unwrap();
        let res = self
            .rest
            .request(method, &url)
            .bearer_auth(token)
            .send()
            .await?;

        Ok(res)
    }

    // TODO
    // replacing body: Option<&T> should work
    pub async fn req_with_body<T: Serialize + ?Sized>(
        &self,
        url: String,
        method: Method,
        body: &T,
    ) -> Result<Response, BrigadeError> {
        let token = self.token.clone().unwrap();
        let res = self
            .rest
            .request(method, &url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        Ok(res)
    }
}

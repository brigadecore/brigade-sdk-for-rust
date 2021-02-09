use crate::error::BrigadeError;
use hyper::Method;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_with::*;

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

    pub async fn req<T: Serialize + ?Sized>(
        &self,
        url: String,
        method: Method,
        body: Option<&T>,
    ) -> Result<Response, BrigadeError> {
        let token = self.token.clone().unwrap();
        let req = self.rest.request(method, &url).bearer_auth(token);
        let res = match body {
            Some(body) => req.json(body).send().await?,
            None => req.send().await?,
        };

        Ok(res)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmptyBody {}

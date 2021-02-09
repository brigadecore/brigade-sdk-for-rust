use crate::error::BrigadeError;
use hyper::{HeaderMap, Method};
use reqwest::{RequestBuilder, Response};
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

#[derive(Debug, Clone)]
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
            token,
            rest: client,
        })
    }

    pub async fn req<T: Serialize + ?Sized, Q: Serialize + ?Sized>(
        &self,
        url: String,
        method: Method,
        body: Option<&T>,
        query: Option<&Q>,
        headers: Option<HeaderMap>,
    ) -> Result<Response, BrigadeError> {
        let req = self.create_req(url, method, body, query, headers);

        let res = req.send().await?;
        Ok(res)
    }

    pub async fn req_with_basic_auth<T: Serialize + ?Sized, Q: Serialize + ?Sized>(
        &self,
        url: String,
        method: Method,
        body: Option<&T>,
        query: Option<&Q>,
        headers: Option<HeaderMap>,
        user: String,
        pwd: Option<String>,
    ) -> Result<Response, BrigadeError> {
        let req = self.create_req(url, method, body, query, headers);
        let res = req.basic_auth(user, pwd).send().await?;
        Ok(res)
    }

    fn create_req<T: Serialize + ?Sized, Q: Serialize + ?Sized>(
        &self,
        url: String,
        method: Method,
        body: Option<&T>,
        query: Option<&Q>,
        headers: Option<HeaderMap>,
    ) -> RequestBuilder {
        let mut req = self.rest.request(method, &url);
        req = match self.token.clone() {
            Some(t) => req.bearer_auth(t),
            None => req,
        };
        req = match body {
            Some(b) => req.json(b),
            None => req,
        };
        req = match query {
            Some(q) => req.query(q),
            None => req,
        };
        req = match headers {
            Some(h) => req.headers(h),
            None => req,
        };

        req
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Empty {}

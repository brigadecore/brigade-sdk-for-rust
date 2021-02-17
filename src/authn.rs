use crate::{
    client::{Client, ClientConfig},
    meta::TypeMeta,
};
use anyhow::Error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    pub value: String,
}

pub struct SessionsClient {
    client: Client,
}

impl SessionsClient {
    pub fn new(address: String, cfg: ClientConfig, token: Option<String>) -> Result<Self, Error> {
        let client = Client::new(address, "sessions".to_string(), cfg, token)?;
        Ok(SessionsClient { client })
    }

    pub async fn create_root_session(&self, pwd: String) -> Result<Token, Error> {
        let url = format!("{}/v2/{}", self.client.base_address, self.client.url_path);
        let res = self
            .client
            .req(Method::POST, &url, None)
            .query(&[("root", "true")])
            .basic_auth(String::from("root"), Some(pwd))
            .send()
            .await?;
        let token: Token = serde_json::from_str(&res.text().await?.to_string())?;
        Ok(token)
    }
}

#[tokio::test]
async fn test_create_root_session() {
    let cfg = ClientConfig {
        allow_insecure_connections: true,
    };
    let sc = SessionsClient::new("https://localhost:8080".to_string(), cfg, None).unwrap();
    let token = sc
        .create_root_session("F00Bar!!!".to_string())
        .await
        .unwrap();

    println!("{:#?}", token);
}

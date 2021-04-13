use crate::meta::{List, ListOptions};
use anyhow::{Error, Result};
use reqwest::{IntoUrl, Method, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use serde_with::*;

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

pub struct Client {
    pub rest: reqwest::Client,
    pub config: ClientConfig,
    pub base_address: String,
    pub token: Option<String>,
    pub url_path: String,
}

impl Client {
    pub fn new(
        address: String,
        url_path: String,
        config: ClientConfig,
        token: Option<String>,
    ) -> Result<Self, Error> {
        let rest = reqwest::Client::builder()
            .danger_accept_invalid_certs(config.allow_insecure_connections)
            .build()?;
        Ok(Self {
            rest,
            url_path,
            config,
            base_address: address,
            token,
        })
    }
}

impl Client {
    pub fn req<U: IntoUrl>(
        &self,
        method: Method,
        url: U,
        opts: Option<ListOptions>,
    ) -> RequestBuilder {
        let mut req = self.rest.request(method, url);

        if let Some(token) = self.token.as_ref() {
            req = req.bearer_auth(token);
        };
        if let Some(opts) = opts {
            if let Some(c) = opts.continue_id {
                req = req.query(&[("continue", c)]);
            }
            if let Some(l) = opts.limit {
                req = req.query(&[("limit", l)]);
            }
        }

        req
    }

    pub async fn get<T: Serialize + DeserializeOwned + Sized>(
        &self,
        id: String,
    ) -> Result<T, Error> {
        let url = format!("{}/v2/{}/{}", self.base_address, self.url_path, id);
        let res = self.req(Method::GET, &url, None).send().await?;
        let obj: T = serde_json::from_str(&res.text().await?)?;
        Ok(obj)
    }

    pub async fn create<
        T: Serialize + DeserializeOwned + Sized + Send + Clone,
        U: Serialize + DeserializeOwned + Sized,
    >(
        &self,
        t: &T,
    ) -> Result<U, Error> {
        let url = format!("{}/v2/{}", self.base_address, self.url_path);
        let res = self.req(Method::POST, &url, None).json(&t).send().await?;
        let res = &res.text().await?;
        println!("{}", res);
        let obj: U = serde_json::from_str(res)?;
        Ok(obj)
    }

    pub async fn update<T: Serialize + DeserializeOwned + Sized + Send + Clone>(
        &self,
        id: String,
        t: &T,
    ) -> Result<T, Error> {
        let url = format!("{}/v2/{}/{}", self.base_address, self.url_path, id);
        let res = self.req(Method::PUT, &url, None).json(&t).send().await?;
        let obj: T = serde_json::from_str(&res.text().await?)?;
        Ok(obj)
    }

    pub async fn delete<T: Serialize + DeserializeOwned + Sized>(
        &self,
        id: String,
    ) -> Result<(), Error> {
        let url = format!("{}/v2/{}/{}", self.base_address, self.url_path, id);
        self.req(Method::DELETE, &url, None).send().await?;
        Ok(())
    }

    pub async fn list<T: Serialize + DeserializeOwned + Sized>(
        &self,
        opts: Option<ListOptions>,
    ) -> Result<List<T>, Error> {
        let url = format!("{}/v2/{}", self.base_address, self.url_path);
        let res = self.req(Method::GET, &url, opts).send().await?;
        let list: List<T> = serde_json::from_str(&res.text().await?)?;
        Ok(list)
    }

    // Specific clients might have selectors for listing clients.
    // This utility method returns a properly formatted request builder
    // back to a specific client, which can then apply any query
    // parameters it needs when listing objects.
    pub fn list_req(&self, opts: Option<ListOptions>) -> RequestBuilder {
        let url = format!("{}/v2/{}", self.base_address, self.url_path);
        self.req(Method::GET, &url, opts)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        authn::{SessionsClient, Token},
        meta::{APIVersion, Kind, TypeMeta},
        projects::Project,
    };

    use super::*;

    #[tokio::test]
    async fn test_get_project() {
        let cl = get_client("projects".to_string()).await;
        let project = cl.get::<Project>("hello-world".to_string()).await.unwrap();
        println!("{:#?}", project);
    }

    #[tokio::test]
    async fn test_create_project() {
        let cl = get_client("projects".to_string()).await;
        let script = r#"
        console.log("Hello, World!")
    "#
        .to_string();
        let mut project = Project::new(
            String::from("hello-rust-sdk"),
            String::from("A project created from the Brigade Rust SDK"),
            script,
        );
        ensure_project_meta(&mut project);
        let project = cl.create::<Project, Project>(&project).await.unwrap();
        println!("{:#?}", project);
    }

    #[tokio::test]
    async fn test_update_project() {
        let cl = get_client("projects".to_string()).await;
        let mut project = cl
            .get::<Project>("hello-rust-sdk".to_string())
            .await
            .unwrap();
        ensure_project_meta(&mut project);
        project.description = Some("totally new descrption".to_string());
        cl.update(project.metadata.id.clone(), &project)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_delete_project() {
        let cl = get_client("projects".to_string()).await;
        cl.delete::<Project>("hello-rust-sdk".to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_list_projects() {
        let cl = get_client("projects".to_string()).await;
        let projects = cl.list::<Project>(None).await.unwrap();
        println!("{:#?}", projects);
    }

    async fn get_client(url_path: String) -> Client {
        let address = "https://localhost:8080";
        let cfg = ClientConfig {
            allow_insecure_connections: true,
        };
        let token = get_token(String::from(address), cfg.clone()).await;
        Client::new(
            String::from(address),
            String::from(url_path),
            cfg,
            Some(token.value),
        )
        .unwrap()
    }

    async fn get_token(address: String, cfg: ClientConfig) -> Token {
        let sc = SessionsClient::new(String::from(address), cfg.clone(), None).unwrap();
        let token = sc
            .create_root_session("F00Bar!!!".to_string())
            .await
            .unwrap();
        token
    }

    fn ensure_project_meta(project: &mut Project) {
        project.type_meta = Some(TypeMeta {
            kind: Kind::Project,
            api_version: APIVersion::V2,
        });

        // These fields should never be sent by a client, and
        // will be rejected by the server.
        project.metadata.created = None;
        project.kubernetes = None;
    }
}

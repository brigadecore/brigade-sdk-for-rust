use crate::{
    meta::{List, ListOptions},
    rest::{self, ClientConfig},
};
use anyhow::{Error, Result};
use reqwest::{Method, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use serde_with::*;

pub struct Client {
    pub rest: rest::Client,
    pub url_path: String,
}

impl Client {
    pub fn new(
        address: String,
        url_path: String,
        cfg: ClientConfig,
        token: Option<String>,
    ) -> Result<Self, Error> {
        let rest = rest::Client::new(address, cfg, token)?;
        Ok(Self { rest, url_path })
    }
}

// #[async_trait::async_trait]
impl Client {
    pub async fn get<
        T: Serialize + DeserializeOwned + Sized,
        U: Serialize + DeserializeOwned + Sized,
    >(
        &self,
        id: String,
    ) -> Result<U, Error> {
        let url = format!("{}/v2/{}/{}", self.rest.address, self.url_path, id);
        let res = self.rest.req(Method::GET, &url, None).send().await?;
        let obj: U = serde_json::from_str(&res.text().await?)?;
        Ok(obj)
    }

    pub async fn create<
        T: Serialize + DeserializeOwned + Sized + Send + Clone,
        U: Serialize + DeserializeOwned + Sized,
    >(
        &self,
        t: &T,
    ) -> Result<U, Error> {
        let url = format!("{}/v2/{}", self.rest.address, self.url_path);
        let res = self
            .rest
            .req(Method::POST, &url, None)
            .json(&t)
            .send()
            .await?;
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
        let url = format!("{}/v2/{}/{}", self.rest.address, self.url_path, id);
        let res = self
            .rest
            .req(Method::PUT, &url, None)
            .json(&t)
            .send()
            .await?;
        let obj: T = serde_json::from_str(&res.text().await?)?;
        Ok(obj)
    }

    pub async fn delete<T: Serialize + DeserializeOwned + Sized>(
        &self,
        id: String,
    ) -> Result<(), Error> {
        let url = format!("{}/v2/{}/{}", self.rest.address, self.url_path, id);
        self.rest.req(Method::DELETE, &url, None).send().await?;
        Ok(())
    }

    pub async fn list<T: Serialize + DeserializeOwned + Sized>(
        &self,
        opts: Option<ListOptions>,
    ) -> Result<List<T>, Error> {
        let url = format!("{}/v2/{}", self.rest.address, self.url_path);
        let res = self.rest.req(Method::GET, &url, opts).send().await?;
        let list: List<T> = serde_json::from_str(&res.text().await?)?;
        Ok(list)
    }

    // Specific client might have selectors for listing clients.
    // This utility method returns a properly formatted request builder
    // back to a specific client, which can then apply any query
    // parameters it needs when listing objects.
    pub fn list_req(&self, opts: Option<ListOptions>) -> RequestBuilder {
        let url = format!("{}/v2/{}", self.rest.address, self.url_path);
        self.rest.req(Method::GET, &url, opts)
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
        let project = cl
            .get::<Project, Project>("hello-world".to_string())
            .await
            .unwrap();
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
            .get::<Project, Project>("hello-rust-sdk".to_string())
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
    async fn test_list_project() {
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

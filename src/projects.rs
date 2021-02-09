use crate::{
    events::EventSubscription,
    meta::{APIVersion, Kind, ObjectMeta, TypeMeta},
    rest::{Client, ClientConfig, EmptyBody},
    worker::WorkerSpec,
};
use anyhow::{Error, Result};
use hyper::Method;
use serde::{Deserialize, Serialize};
use serde_with::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub metadata: ObjectMeta,
    #[serde(flatten)]
    pub type_meta: Option<TypeMeta>,

    pub description: Option<String>,
    pub spec: ProjectSpec,
    pub kubernetes: Option<KubernetesDetails>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSpec {
    pub event_subscriptions: Option<Vec<EventSubscription>>,
    pub worker_template: WorkerSpec,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KubernetesDetails {
    namespace: Option<String>,
}

pub struct ProjectsClient {
    pub client: Client,
}

impl ProjectsClient {
    pub fn new(address: String, cfg: ClientConfig, token: Option<String>) -> Result<Self, Error> {
        let client = Client::new(address, cfg, token)?;
        Ok(Self { client })
    }

    pub async fn get(&self, id: String) -> Result<Project, Error> {
        let url = format!("{}/v2/projects/{}", self.client.address, id);
        let res = self
            .client
            .req(url, Method::GET, None::<&EmptyBody>)
            .await?;
        let project: Project = serde_json::from_str(&res.text().await?.to_string())?;
        Ok(project)
    }

    pub async fn create(&self, project: &Project) -> Result<Project, Error> {
        let url = format!("{}/v2/projects", self.client.address);
        let mut project = project.clone();
        self.ensure_project_meta(&mut project);
        let res = self.client.req(url, Method::POST, Some(&project)).await?;
        let project: Project = serde_json::from_str(&res.text().await?.to_string())?;
        Ok(project)
    }

    pub async fn update(&self, project: &Project) -> Result<Project, Error> {
        let url = format!(
            "{}/v2/projects/{}",
            self.client.address, project.metadata.id
        );
        let mut project = project.clone();
        self.ensure_project_meta(&mut project);
        let res = self.client.req(url, Method::PUT, Some(&project)).await?;
        let str = &res.text().await?.to_string();
        let project: Project = serde_json::from_str(str)?;
        Ok(project)
    }

    fn ensure_project_meta(&self, project: &mut Project) {
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        authn::SessionsClient,
        authn::Token,
        meta::ObjectMeta,
        projects::{Project, ProjectSpec, ProjectsClient},
        rest::ClientConfig,
        worker::WorkerSpec,
    };

    #[tokio::test]
    async fn test_get_project() {
        let address = "https://localhost:8080";
        let cfg = ClientConfig {
            allow_insecure_connections: true,
        };
        let token = get_token(String::from(address), cfg.clone()).await;
        let pc = ProjectsClient::new(String::from(address), cfg, Some(token.value)).unwrap();
        let p = pc.get("hello-world".to_string()).await.unwrap();
        println!("{:#?}", p);
    }

    #[tokio::test]
    async fn test_create_project() {
        let address = "https://localhost:8080";
        let cfg = ClientConfig {
            allow_insecure_connections: true,
        };
        let token = get_token(String::from(address), cfg.clone()).await;
        let pc = ProjectsClient::new(String::from(address), cfg, Some(token.value)).unwrap();

        let mut default_config_files: HashMap<String, String> = HashMap::new();
        let val = r#"
        console.log("Hello, World!")
    "#;
        default_config_files.insert("brigade.js".to_string(), val.to_string());
        let project = Project {
            metadata: ObjectMeta {
                id: String::from("hello-rust-sdk"),
                created: None,
            },
            type_meta: None,
            description: Some(String::from("A project created from the Brigade Rust SDK")),
            spec: ProjectSpec {
                event_subscriptions: None,
                worker_template: WorkerSpec {
                    container: None,
                    use_workspace: None,
                    workspace_size: None,
                    git: None,
                    job_policies: None,
                    log_level: None,
                    config_files_directory: None,
                    default_config_files: Some(default_config_files),
                },
            },
            kubernetes: None,
        };

        let project = pc.create(&project).await.unwrap();
        println!("{:#?}", project);
    }

    #[tokio::test]
    async fn test_update_project() {
        let address = "https://localhost:8080";
        let cfg = ClientConfig {
            allow_insecure_connections: true,
        };
        let token = get_token(String::from(address), cfg.clone()).await;
        let pc = ProjectsClient::new(String::from(address), cfg, Some(token.value)).unwrap();
        let mut p = pc.get("hello-rust-sdk".to_string()).await.unwrap();
        p.description = Some("totally new descrption".to_string());

        pc.update(&p).await.unwrap();
    }

    async fn get_token(address: String, cfg: ClientConfig) -> Token {
        let sc = SessionsClient::new(String::from(address), cfg.clone(), None).unwrap();
        let token = sc
            .create_root_session("F00Bar!!!".to_string())
            .await
            .unwrap();

        token
    }
}

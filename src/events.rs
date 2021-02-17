use crate::{
    client::Client,
    client::ClientConfig,
    meta::{APIVersion, Kind, List, ListOptions, ObjectMeta, TypeMeta},
    worker::{Worker, WorkerPhase},
};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub metadata: Option<ObjectMeta>,
    #[serde(flatten)]
    pub type_meta: Option<TypeMeta>,

    #[serde(rename = "projectID")]
    pub project_id: String,
    pub source: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub labels: Option<HashMap<String, String>>,
    pub short_title: Option<String>,
    pub long_title: Option<String>,
    pub git: Option<GitDetails>,
    pub payload: Option<String>,
    pub worker: Option<Worker>,
}

impl Event {
    pub fn new(project_id: String, source: String, event_type: String) -> Self {
        Self {
            metadata: None,
            type_meta: None,
            project_id: project_id,
            source,
            event_type,
            labels: None,
            short_title: None,
            long_title: None,
            git: None,
            payload: None,
            worker: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventSubscription {
    pub source: String,
    pub types: Vec<String>,
    pub labels: HashMap<String, String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventsSelector {
    pub project_id: Option<String>,
    pub worker_phases: Option<Vec<WorkerPhase>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitDetails {
    #[serde(rename = "cloneURL")]
    pub clone_url: Option<String>,
    pub commit: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelManyEventsResult {
    pub count: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteManyEventsResult {
    pub count: i64,
}

pub struct EventsClient {
    pub client: Client,
}

impl EventsClient {
    pub fn new(address: String, cfg: ClientConfig, token: Option<String>) -> Result<Self, Error> {
        let client = Client::new(address, "events".to_string(), cfg, token)?;
        Ok(Self { client })
    }

    pub async fn get(&self, id: String) -> Result<Event, Error> {
        let event = self.client.get::<Event>(id).await?;
        Ok(event)
    }

    pub async fn list(
        &self,
        sel: Option<EventsSelector>,
        opts: Option<ListOptions>,
    ) -> Result<List<Event>, Error> {
        let mut req = self.client.list_req(opts);
        if let Some(s) = sel {
            if let Some(id) = s.project_id {
                req = req.query(&[("projectID", id)]);
            }
            if let Some(p) = s.worker_phases {
                // TODO
                //
                // There is an issue with serializing a Vec<WorkerPhase>, and the events selector
                // doesn't currently work.
                // req = req.query(&[("workerPhases", &WorkerPhase::vec_to_query_param(p)?)]);
                // req = req.query(&[("workerPhases", "SUCCEEDED")]);
            }
        };

        let res = req.send().await?;
        let str = &res.text().await?;
        let events: List<Event> = serde_json::from_str(str)?;
        Ok(events)
    }
    pub async fn create(&self, event: &Event) -> Result<List<Event>, Error> {
        let mut event = event.clone();
        self.ensure_event_meta(&mut event);
        let events = self.client.create::<Event, List<Event>>(&event).await?;
        Ok(events)
    }

    pub async fn cancel(&self, id: String) -> Result<(), Error> {
        let url = format!(
            "{}/v2/{}/{}/cancellation",
            self.client.base_address, self.client.url_path, id
        );
        let res = self
            .client
            .req(reqwest::Method::PUT, &url, None)
            .send()
            .await?;
        println!("{}: {}", res.status().to_string(), res.text().await?);
        Ok(())
    }

    fn ensure_event_meta(&self, event: &mut Event) {
        event.type_meta = Some(TypeMeta {
            kind: Kind::Event,
            api_version: APIVersion::V2,
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        authn::{SessionsClient, Token},
        client::ClientConfig,
    };

    #[tokio::test]
    async fn test_get_event() {
        let ec = get_events_client().await;
        let e = ec
            .get("c325bca8-c615-4061-88ab-25aab9000de7".to_string())
            .await
            .unwrap();
        println!("{:#?}", e);
    }

    #[tokio::test]
    async fn test_list_events() {
        let ec = get_events_client().await;
        let el = ec.list(None, None).await.unwrap();
        println!("{:#?}", el);
    }

    #[tokio::test]
    async fn test_list_events_with_sel_project() {
        let ec = get_events_client().await;
        let sel = EventsSelector {
            project_id: Some(String::from("hello-world")),
            worker_phases: None,
        };
        let el = ec.list(Some(sel), None).await.unwrap();
        println!("{:#?}", el);
    }

    #[tokio::test]
    async fn test_list_events_with_sel_phases() {
        let ec = get_events_client().await;
        let sel = EventsSelector {
            project_id: None,
            worker_phases: Some(vec![WorkerPhase::Succeeded]),
        };
        let el = ec.list(Some(sel), None).await.unwrap();
        println!("{:#?}", el);
    }

    #[tokio::test]
    async fn test_create_event() {
        let ec = get_events_client().await;
        let ev = Event::new(
            "hello-world".to_string(),
            "rust-sdk".to_string(),
            "rust-sdk-test".to_string(),
        );
        let res = ec.create(&ev).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn test_cancel_event() {
        let ec = get_events_client().await;
        ec.cancel("7cf24d38-f2b2-4d0f-9aac-c0a07c01c78c".to_string())
            .await
            .unwrap();
    }

    async fn get_token(address: String, cfg: ClientConfig) -> Token {
        let sc = SessionsClient::new(String::from(address), cfg.clone(), None).unwrap();
        let token = sc
            .create_root_session("F00Bar!!!".to_string())
            .await
            .unwrap();
        token
    }

    async fn get_events_client() -> EventsClient {
        let address = "https://localhost:8080";
        let cfg = ClientConfig {
            allow_insecure_connections: true,
        };
        let token = get_token(String::from(address), cfg.clone()).await;
        EventsClient::new(String::from(address), cfg, Some(token.value)).unwrap()
    }
}

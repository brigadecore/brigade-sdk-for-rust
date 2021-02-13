use crate::{
    meta::{ObjectMeta, TypeMeta},
    rest::Client,
    worker::{Worker, WorkerPhase},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub metadata: ObjectMeta,
    #[serde(flatten)]
    pub type_meta: Option<TypeMeta>,

    pub project_id: Option<String>,
    pub source: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub labels: HashMap<String, String>,
    pub short_title: Option<String>,
    pub long_title: Option<String>,
    pub git: Option<GitDetails>,
    pub payload: Option<String>,
    pub worker: Option<Worker>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventSubscription {
    pub source: String,
    pub types: Vec<String>,
    pub labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventsSelector {
    pub project_id: Option<String>,
    pub worker_phases: Option<Vec<WorkerPhase>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitDetails {
    #[serde(rename = "cloneURL")]
    pub clone_url: Option<String>,
    pub commit: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelManyEventsResult {
    pub count: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteManyEventsResult {
    pub count: i64,
}

pub struct EventsClient {
    pub client: Client,
}

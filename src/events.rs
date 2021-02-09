use crate::{meta::ObjectMeta, worker::WorkerPhase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    metadata: Option<ObjectMeta>,
    worker_phases: Option<Vec<WorkerPhase>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventSubscription {
    source: String,
    types: Vec<String>,
    labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventsSelector {
    project_id: Option<String>,
    worker_phases: Option<Vec<WorkerPhase>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitDetails {
    clone_url: Option<String>,
    commit: Option<String>,
    #[serde(rename = "ref")]
    reference: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelManyEventsResult {
    count: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteManyEventsResult {
    count: i32,
}

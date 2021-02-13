use crate::container::ContainerSpec;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum JobPhase {
    #[serde(rename = "ABORTED")]
    Aborted,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "SCHEDULING_FAILED")]
    SchedulingFailed,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "TIMED_OUT")]
    TimedOut,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    pub started: Option<DateTime<Utc>>,
    pub ended: Option<DateTime<Utc>>,
    pub phase: Option<JobPhase>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobHost {
    pub os: Option<String>,
    pub node_selector: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobContainerSpec {
    #[serde(flatten)]
    pub container_spec: ContainerSpec,
    pub working_directory: Option<String>,
    pub workspace_mount_path: Option<String>,
    pub source_mount_path: Option<String>,
    pub privileged: Option<bool>,
    pub use_host_docker_socket: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobSpec {
    pub primary_container: JobContainerSpec,
    pub sidecar_containers: Option<HashMap<String, JobContainerSpec>>,
    pub timeout_seconds: Option<i64>,
    pub host: Option<JobHost>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub spec: JobSpec,
    pub status: Option<JobStatus>,
}

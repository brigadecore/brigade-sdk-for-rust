use crate::container::ContainerSpec;

use serde::{Deserialize, Serialize};
use serde_with::*;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkerSpec {
    pub container: Option<ContainerSpec>,
    pub use_workspace: Option<bool>,
    pub workspace_size: Option<String>,
    pub git: Option<GitConfig>,
    pub job_policies: Option<JobPolicies>,
    pub log_level: Option<LogLevel>,
    pub config_files_directory: Option<String>,
    pub default_config_files: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitConfig {
    #[serde(rename = "cloneURL")]
    commit: Option<String>,
    #[serde(rename = "ref")]
    reference: Option<String>,
    init_submodules: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KubernetesConfig {
    image_pull_secrets: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JobPolicies {
    allow_provileged: Option<bool>,
    allow_docker_soecket_mount: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum LogLevel {
    #[serde(rename = "DEBUG")]
    Debug,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "WARN")]
    Warn,
    #[serde(rename = "ERROR")]
    Error,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum WorkerPhase {
    #[serde(rename = "ABORTED")]
    Aborted,
    #[serde(rename = "CANCELED")]
    Canceled,
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

use crate::{container::ContainerSpec, job::Job};
use anyhow::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Worker {
    pub spec: WorkerSpec,
    pub status: WorkerStatus,
    pub jobs: Option<HashMap<String, Job>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkerSpec {
    pub container: Option<ContainerSpec>,
    pub use_workspace: Option<bool>,
    pub workspace_size: Option<String>,
    pub git: Option<GitConfig>,
    pub kubernetes: Option<KubernetesConfig>,
    pub job_policies: Option<JobPolicies>,
    pub log_level: Option<LogLevel>,
    pub config_files_directory: Option<String>,
    pub default_config_files: Option<HashMap<String, String>>,
}

impl WorkerSpec {
    pub fn new(script: String) -> Self {
        let mut default_config_files: HashMap<String, String> = HashMap::new();
        default_config_files.insert("brigade.js".to_string(), script);

        WorkerSpec {
            container: None,
            use_workspace: None,
            workspace_size: None,
            git: None,
            kubernetes: None,
            job_policies: None,
            log_level: None,
            config_files_directory: None,
            default_config_files: Some(default_config_files),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkerStatus {
    pub started: Option<DateTime<Utc>>,
    pub ended: Option<DateTime<Utc>>,
    pub phase: Option<WorkerPhase>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitConfig {
    #[serde(rename = "cloneURL")]
    pub clone_url: Option<String>,
    pub commit: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    pub init_submodules: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KubernetesConfig {
    image_pull_secrets: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobPolicies {
    allow_provileged: Option<bool>,
    allow_docker_soecket_mount: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
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

impl WorkerPhase {
    pub fn vec_to_query_param(vec: Vec<Self>) -> Result<String, Error> {
        // let mut phases: Vec<String> = Vec::new();
        // for p in vec.iter() {
        //     let str = serde_json::to_string(p)?;
        //     phases.push(str);
        // }
        // vec.iter().enumerate().map(|(i, &x)| {});
        // Ok(phases.join(","))
        todo!()
    }
}

#[test]
fn test_vec() {
    let v = vec![WorkerPhase::Succeeded];
    let str = WorkerPhase::vec_to_query_param(v).unwrap();
    println!("{}", &str);
}

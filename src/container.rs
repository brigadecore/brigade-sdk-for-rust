use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerSpec {
    image: String,
    image_pull_policy: Option<ImagePullPolicy>,
    command: Option<Vec<String>>,
    arguments: Option<Vec<String>>,
    environment: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ImagePullPolicy {
    IfNotPresent,
    Always,
}

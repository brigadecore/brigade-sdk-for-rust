use serde::{Deserialize, Serialize};
use serde_with::*;
#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    pub id: String,
    pub created: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeta {
    pub kind: Kind,
    pub api_version: APIVersion,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum APIVersion {
    #[serde(rename = "brigade.sh/v2")]
    V2,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Kind {
    Token,
    Project,
}

#[test]
fn test_type_meta_serialization() {
    let tm = TypeMeta {
        api_version: APIVersion::V2,
        kind: Kind::Token,
    };

    let str = serde_json::to_string(&tm).unwrap();
    let tmu: TypeMeta = serde_json::from_str(&str).unwrap();
    assert_eq!(tmu.kind, Kind::Token);
    assert_eq!(tmu.api_version, APIVersion::V2);
}

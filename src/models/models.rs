use serde::Deserialize;
use serde::Serialize;

//main model -- Version Manifest Model
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionManifestModel {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: String,
    pub compliance_level: i32,
}

//others...
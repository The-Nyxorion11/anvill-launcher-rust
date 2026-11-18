use serde::Deserialize;



//main model -- Version Manifest Model
#[derive(Deserialize, Debug)]
pub struct VersionManifestModel {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Deserialize, Debug)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Debug)]
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
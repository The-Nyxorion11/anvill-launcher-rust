
use reqwest::Error;
use crate::models::models::VersionManifestModel;

// Get version manifest from Mojang API
pub async fn mojang_service() -> Result<VersionManifestModel, Error> {
    const URL: &str ="https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

    // Get response from Mojang API
    let response = reqwest::get(URL)
        .await?
        .error_for_status()?;

    
    // Deserialize response
    let body: VersionManifestModel = response.json().await?;

    println!("Response body: {:#?}", body);

    Ok(body)
}

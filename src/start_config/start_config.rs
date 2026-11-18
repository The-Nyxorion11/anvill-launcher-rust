
use crate::services::mojang_service::mojang_service;

pub async fn start_config() {
    version_manifest_config().await;
}

async fn version_manifest_config() {
    let version_manifest = match mojang_service().await {
        Ok(manifest) => {
            println!("Manifest downloaded successfully!");
            manifest
        }

        Err(e) => {
            eprintln!("Error fetching version manifest: {}", e);
            return;
        }
    };

    println!(
        "Último release: {}",
        version_manifest.latest.release
    );
}

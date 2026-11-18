use crate::services::mojang_service::mojang_service;
use crate::start_config::config_file::create_files;
use serde_json::json;
use std::path::PathBuf; 
use std::fs;


pub async fn start_config() {
    let path = create_files();
    version_manifest_config(path).await;
}

async fn version_manifest_config(path: PathBuf) {
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

    let manifest_string = serde_json::to_string_pretty(&version_manifest).unwrap();
    fs::write(path.join(path.join("versions/version_manifest_v2.json")),manifest_string).unwrap();
}


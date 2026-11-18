mod models;
mod services;

#[path = "start_config/start_config.rs"]
mod start_config;

#[tokio::main]
async fn main() {
    start_config::start_config().await;
}
mod models;
mod services;

mod start_config;

#[tokio::main]
async fn main() {
    start_config::start_config().await;
}
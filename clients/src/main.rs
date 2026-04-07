use std::sync::Arc;

use crate::{api_client::ApiClient, personalities::admin::Admin};

pub mod api_client;
pub mod helpers;
pub mod models;
pub mod personalities;

pub const PASSWORD: &str = "123456";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let client = Arc::new(ApiClient::new("http://127.0.0.1:3000".to_string()));
    let mut admin = Admin::new(client);
    admin.run().await;
}

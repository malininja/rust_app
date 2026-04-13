use std::{env, sync::Arc};

use crate::{api_client::ApiClient, personalities::admin::Admin};

pub mod api_client;
pub mod errors;
pub mod helpers;
pub mod models;
pub mod personalities;

pub const PASSWORD: &str = "123456";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let api_url = env::var("API_URL").unwrap();
    let client = Arc::new(ApiClient::new(api_url));
    let mut admin = Admin::new(client);
    admin.run().await;
}

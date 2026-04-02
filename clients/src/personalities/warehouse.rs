use std::{sync::Arc, time::Duration};

use rand::{SeedableRng, seq::IndexedRandom};

use crate::api_client::ApiClient;

const LOG_CONTEXT: &str = "WAREHOUSE";

pub struct Warehouse {
    pub name: String,
    client: Arc<ApiClient>,
    token: Option<String>,
}

impl Warehouse {
    pub fn new(name: String, client: Arc<ApiClient>) -> Self {
        Self {
            name,
            client,
            token: None,
        }
    }

    pub async fn run(self) {
        let mut rng = rand::rngs::StdRng::from_rng(&mut rand::rng());
        let numbers: Vec<u64> = (10..20).collect();

        loop {
            tracing::info!(
                "{}. user: {} - use vars: {:?}{:?}",
                LOG_CONTEXT,
                self.name,
                self.client,
                self.token
            );

            let seconds = numbers.choose(&mut rng).unwrap();
            tokio::time::sleep(Duration::from_secs(*seconds)).await;
        }
    }
}

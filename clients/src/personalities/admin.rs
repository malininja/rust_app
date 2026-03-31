use std::{sync::Arc, time::Duration};

use rand::seq::IndexedRandom;

use crate::api_client::ApiClient;

const LOG_CONTEXT: &str = "Admin";

pub struct Admin {
    client: Arc<ApiClient>,
    auth_token: Option<String>,
}

impl Admin {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            client,
            auth_token: None,
        }
    }

    pub async fn run(&mut self) {
        let mut rng = rand::rng();
        let numbers: Vec<u64> = (1..15).collect();

        loop {
            self.run_once().await;

            let seconds = numbers.choose(&mut rng).unwrap();
            tokio::time::sleep(Duration::from_secs(*seconds)).await;
        }
    }

    async fn run_once(&mut self) {
        if let Some(token) = self.token_get().await {
            let users_option = match self.client.users_get(&token).await {
                Ok(users) => Some(users),
                Err(e) => {
                    tracing::error!("{}. users_fetch error: {}", LOG_CONTEXT, e);
                    None
                }
            };

            if let Some(users) = users_option {
                println!("{:?}", users);
            }
        }
    }

    async fn token_get(&mut self) -> Option<String> {
        match &self.auth_token {
            Some(t) => Some(t.clone()),
            None => {
                let t_option = self.token_fetch().await;

                if let Some(t) = &t_option {
                    self.auth_token = Some(t.clone());
                }

                t_option
            }
        }
    }

    async fn token_fetch(&self) -> Option<String> {
        match self
            .client
            .login("admin".to_string(), "123456".to_string())
            .await
        {
            Ok(token) => Some(token),
            Err(e) => {
                tracing::error!("{}. token_fetch error: {}", LOG_CONTEXT, e);
                None
            }
        }
    }
}

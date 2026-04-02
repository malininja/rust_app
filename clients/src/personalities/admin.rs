use std::{collections::HashMap, sync::Arc, time::Duration};

use rand::seq::IndexedRandom;
use tokio::task::{AbortHandle, JoinSet};

use crate::{
    api_client::ApiClient,
    helpers::generate_random_string,
    models::{
        roles_enum::Role, user_create_dto::UserCreateDto, user_response_dto::UserResponseDto,
    },
    personalities::sales::Sales,
};

const LOG_CONTEXT: &str = "ADMIN";
const PASSWORD: &str = "123456";

pub struct Admin {
    client: Arc<ApiClient>,
    auth_token: Option<String>,
    sales_processes: JoinSet<()>,
    sales_aborthandles: HashMap<String, AbortHandle>,
}

impl Admin {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            client,
            auth_token: None,
            sales_processes: JoinSet::new(),
            sales_aborthandles: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        let mut rng = rand::rng();
        let numbers: Vec<u64> = (20..40).collect();

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
                let role_id: i32 = Role::Sales.into();
                let sales_users: Vec<&UserResponseDto> =
                    users.iter().filter(|u| u.role_id == role_id).collect();

                for sales_user in &sales_users {
                    if !self.sales_aborthandles.contains_key(&sales_user.username) {
                        let sales = Sales::new(sales_user.username.clone(), self.client.clone());
                        let aborthandle = self.sales_processes.spawn(sales.run());
                        self.sales_aborthandles
                            .insert(sales_user.username.clone(), aborthandle);

                        tracing::info!(
                            "{}. New sales agent added. {}",
                            LOG_CONTEXT,
                            &sales_user.username
                        );
                    }
                }

                if sales_users.len() < 3 {
                    let create_dto = UserCreateDto {
                        role_id,
                        username: generate_random_string(8),
                        password: PASSWORD.to_string(),
                    };

                    match self.client.user_create(&token, create_dto).await {
                        Ok(res) => {
                            tracing::info!(
                                "{}. Sales user successfully created in the backend. Name = {}",
                                LOG_CONTEXT,
                                res.username
                            );
                        }
                        Err(e) => {
                            tracing::error!("{}. user_create error. error: {}", LOG_CONTEXT, e);
                        }
                    };
                }
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
            .login("admin".to_string(), PASSWORD.to_string())
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

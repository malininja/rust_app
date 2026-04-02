use std::{collections::HashMap, pin::Pin, sync::Arc, time::Duration};

use rand::seq::IndexedRandom;
use tokio::task::{AbortHandle, JoinSet};

use crate::{
    api_client::ApiClient,
    helpers::generate_random_string,
    models::{
        roles_enum::Role, user_create_dto::UserCreateDto, user_response_dto::UserResponseDto,
    },
    personalities::{sales::Sales, warehouse::Warehouse},
};

const LOG_CONTEXT: &str = "ADMIN";
const PASSWORD: &str = "123456";

pub struct Admin {
    client: Arc<ApiClient>,
    auth_token: Option<String>,
    sales_processes: JoinSet<()>,
    sales_aborthandles: HashMap<String, AbortHandle>,
    warehouse_processes: JoinSet<()>,
    warehouse_aborthandles: HashMap<String, AbortHandle>,
}

impl Admin {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            client,
            auth_token: None,
            sales_processes: JoinSet::new(),
            sales_aborthandles: HashMap::new(),
            warehouse_processes: JoinSet::new(),
            warehouse_aborthandles: HashMap::new(),
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
                self.handle_users(Role::Sales, &users).await;
                self.handle_users(Role::Warehouse, &users).await;
            }
        }
    }

    async fn handle_users(&mut self, role: Role, all_users: &[UserResponseDto]) {
        let role_id: i32 = (&role).into();
        let users: Vec<&UserResponseDto> =
            all_users.iter().filter(|u| u.role_id == role_id).collect();

        for user in &users {
            self.create_user(user.username.clone(), &role);
        }

        if users.len() < 3 {
            self.create_api_user(&role).await;
        }
    }

    async fn create_api_user(&mut self, role: &Role) {
        if let Some(token) = self.token_get().await {
            let dto = UserCreateDto {
                role_id: role.into(),
                username: generate_random_string(8),
                password: PASSWORD.to_string(),
            };

            match self.client.user_create(&token, dto).await {
                Ok(res) => {
                    tracing::info!(
                        "{}. {:?} user successfully created in the backend. Name = {}",
                        LOG_CONTEXT,
                        role,
                        res.username
                    );
                }
                Err(err) => {
                    tracing::error!("{}. user_create error. error: {}", LOG_CONTEXT, err);
                }
            }
        }
    }

    fn create_user(&mut self, username: String, role: &Role) {
        let handles_processes = match role {
            Role::Sales => (&mut self.sales_aborthandles, &mut self.sales_processes),
            Role::Warehouse => (
                &mut self.warehouse_aborthandles,
                &mut self.warehouse_processes,
            ),
            _ => {
                tracing::error!(
                    "{}. Error adding new agent. Invalid role: {:?}",
                    LOG_CONTEXT,
                    role
                );
                return;
            }
        };

        let (aborthandles, processes) = handles_processes;

        if !aborthandles.contains_key(&username) {
            let user_future = match role {
                Role::Sales => Box::pin(Sales::new(username.clone(), self.client.clone()).run())
                    as Pin<Box<dyn Future<Output = ()> + Send>>,

                Role::Warehouse => {
                    Box::pin(Warehouse::new(username.clone(), self.client.clone()).run())
                        as Pin<Box<dyn Future<Output = ()> + Send>>
                }

                _ => return,
            };

            let aborthandle = processes.spawn(user_future);
            aborthandles.insert(username.clone(), aborthandle);
                              
                                        tracing::info!(
                                "{}. New {:?} agent added. {}",
                                LOG_CONTEXT,
                                role,
                                &username
                            );
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

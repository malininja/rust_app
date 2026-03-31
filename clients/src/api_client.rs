use anyhow::Context;

use crate::models::{
    login_request_dto::LoginRequestDto, login_response_dto::LoginResponseDto,
    user_response_dto::UserResponseDto,
};

const ERROR_CONTEXT: &str = "ApiClient";

pub struct ApiClient {
    client: reqwest::Client,
    login_url: String,
    users_get_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            login_url: format!("{}/login", &base_url),
            users_get_url: format!("{}/users", &base_url),
        }
    }

    pub async fn login(&self, username: String, password: String) -> anyhow::Result<String> {
        let login_dto = LoginRequestDto { username, password };

        let auth_res = self
            .client
            .post(&self.login_url)
            .json(&login_dto)
            .send()
            .await
            .with_context(|| format!("{}. login error", ERROR_CONTEXT))?;

        let dto = auth_res
            .json::<LoginResponseDto>()
            .await
            .with_context(|| format!("{}. login error", ERROR_CONTEXT))?;
        Ok(dto.token)
    }

    pub async fn users_get(&self, token: &str) -> anyhow::Result<Vec<UserResponseDto>> {
        let response = self
            .client
            .get(&self.users_get_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .with_context(|| format!("{}. get_users error", ERROR_CONTEXT))?;

        response
            .json::<Vec<UserResponseDto>>()
            .await
            .with_context(|| format!("{}. get_users error", ERROR_CONTEXT))
    }
}

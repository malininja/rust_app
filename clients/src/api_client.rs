use serde::{Serialize, de::DeserializeOwned};

use crate::models::{
    article_create_dto::ArticleCreateDto, article_response_dto::ArticleResponseDto,
    login_request_dto::LoginRequestDto, login_response_dto::LoginResponseDto,
    user_create_dto::UserCreateDto, user_response_dto::UserResponseDto,
};

const AUTH_HEADER_KEY: &str = "Authorization";

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    login_url: String,
    users_base_url: String,
    articles_url: String,
}

fn create_auth_header(token: &str) -> String {
    format!("Bearer {}", token)
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            login_url: format!("{}/login", &base_url),
            users_base_url: format!("{}/users", &base_url),
            articles_url: format!("{}/articles", &base_url),
        }
    }

    pub async fn login(&self, username: String, password: String) -> anyhow::Result<String> {
        let login_dto = LoginRequestDto { username, password };

        let dto: LoginResponseDto = self.post(&self.login_url, "", &login_dto).await?;

        Ok(dto.token)
    }

    pub async fn users_get(&self, token: &str) -> anyhow::Result<Vec<UserResponseDto>> {
        self.get(&self.users_base_url, token).await
    }

    pub async fn user_create(
        &self,
        token: &str,
        user: UserCreateDto,
    ) -> anyhow::Result<UserResponseDto> {
        self.post(&self.users_base_url, token, &user).await
    }

    pub async fn articles_get(&self, token: &str) -> anyhow::Result<Vec<ArticleResponseDto>> {
        self.get(&self.articles_url, token).await
    }

    pub async fn article_create(
        &self,
        token: &str,
        article: ArticleCreateDto,
    ) -> anyhow::Result<ArticleResponseDto> {
        self.post(&self.articles_url, token, &article).await
    }

    async fn get<T: DeserializeOwned>(&self, url: &str, token: &str) -> anyhow::Result<T> {
        let res = self
            .client
            .get(url)
            .header(AUTH_HEADER_KEY, create_auth_header(token))
            .send()
            .await?;

        Ok(res.json::<T>().await?)
    }

    async fn post<T: Serialize, U: DeserializeOwned>(
        &self,
        url: &str,
        token: &str,
        payload: &T,
    ) -> anyhow::Result<U> {
        let res = self
            .client
            .post(url)
            .header(AUTH_HEADER_KEY, create_auth_header(token))
            .json(payload)
            .send()
            .await?;

        Ok(res.json::<U>().await?)
    }
}

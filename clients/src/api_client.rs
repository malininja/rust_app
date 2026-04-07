use anyhow::anyhow;

use reqwest::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::models::{
    article_create_dto::ArticleCreateDto, article_response_dto::ArticleResponseDto,
    goods_receipt_head_create_dto::GoodsReceiptHeadCreateDto,
    goods_receipt_head_response_dto::GoodsReceiptHeadResponseDto,
    login_request_dto::LoginRequestDto, login_response_dto::LoginResponseDto,
    user_create_dto::UserCreateDto, user_response_dto::UserResponseDto,
    warehouse_stock_response_dto::WarehouseStockResponseDto,
};

const AUTH_HEADER_KEY: &str = "Authorization";

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    login_url: String,
    users_base_url: String,
    articles_url: String,
    goods_receipt_url: String,
    warehouse_stock_url: String,
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
            goods_receipt_url: format!("{}/goods_receipts", &base_url),
            warehouse_stock_url: format!("{}/warehouse_stocks", &base_url),
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

    pub async fn goods_receipts_get(
        &self,
        token: &str,
    ) -> anyhow::Result<Vec<GoodsReceiptHeadResponseDto>> {
        self.get(&self.goods_receipt_url, token).await
    }

    pub async fn goods_receipt_create(
        &self,
        token: &str,
        goods_receipt: GoodsReceiptHeadCreateDto,
    ) -> anyhow::Result<GoodsReceiptHeadResponseDto> {
        self.post(&self.goods_receipt_url, token, &goods_receipt)
            .await
    }

    pub async fn goods_receipt_confirm(&self, token: &str, id: &Uuid) -> anyhow::Result<()> {
        let res = self
            .client
            .patch(format!("{}/{}/confirm", &self.goods_receipt_url, id))
            .header(AUTH_HEADER_KEY, create_auth_header(token))
            .send()
            .await?;

        if res.status() != StatusCode::NO_CONTENT {
            return Err(anyhow!(
                "goods_receipt_confirm. Invalid status code: {}",
                res.status()
            ));
        }

        Ok(())
    }

    pub async fn goods_receipt_delete(&self, token: &str, id: &Uuid) -> anyhow::Result<()> {
        self.delete(&format!("{}/{}", &self.goods_receipt_url, id), token)
            .await
    }

    pub async fn warehouse_stocks_get(
        &self,
        token: &str,
    ) -> anyhow::Result<Vec<WarehouseStockResponseDto>> {
        self.get(&self.warehouse_stock_url, token).await
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

    async fn delete(&self, url: &str, token: &str) -> anyhow::Result<()> {
        let res = self
            .client
            .delete(url)
            .header(AUTH_HEADER_KEY, create_auth_header(token))
            .send()
            .await?;

        if res.status() != StatusCode::NO_CONTENT {
            return Err(anyhow!(
                "goods_receipt_delete. Invalid status code: {}",
                res.status()
            ));
        }

        Ok(())
    }
}

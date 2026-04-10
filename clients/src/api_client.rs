use reqwest::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    models::{
        article_create_dto::ArticleCreateDto, article_response_dto::ArticleResponseDto,
        goods_receipt_head_create_dto::GoodsReceiptHeadCreateDto,
        goods_receipt_head_response_dto::GoodsReceiptHeadResponseDto,
        invoice_head_create_dto::InvoiceHeadCreateDto,
        invoice_head_response_dto::InvoiceHeadResponseDto, login_request_dto::LoginRequestDto,
        login_response_dto::LoginResponseDto, user_create_dto::UserCreateDto,
        user_response_dto::UserResponseDto,
        warehouse_stock_response_dto::WarehouseStockResponseDto,
    },
};

const LOG_CONTEXT: &str = "ApiClient";
const AUTH_HEADER_KEY: &str = "Authorization";

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    login_url: String,
    users_base_url: String,
    articles_url: String,
    goods_receipt_url: String,
    warehouse_stock_url: String,
    invoice_url: String,
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
            invoice_url: format!("{}/invoices", &base_url),
        }
    }

    pub async fn login(&self, username: String, password: String) -> Result<String, ApiError> {
        let login_dto = LoginRequestDto { username, password };

        let dto: LoginResponseDto = self.post(&self.login_url, "", &login_dto).await?;

        Ok(dto.token)
    }

    pub async fn users_get(&self, token: &str) -> Result<Vec<UserResponseDto>, ApiError> {
        self.get(&self.users_base_url, token).await
    }

    pub async fn user_create(
        &self,
        token: &str,
        user: UserCreateDto,
    ) -> Result<UserResponseDto, ApiError> {
        self.post(&self.users_base_url, token, &user).await
    }

    pub async fn articles_get(&self, token: &str) -> Result<Vec<ArticleResponseDto>, ApiError> {
        self.get(&self.articles_url, token).await
    }

    pub async fn article_create(
        &self,
        token: &str,
        article: ArticleCreateDto,
    ) -> Result<ArticleResponseDto, ApiError> {
        self.post(&self.articles_url, token, &article).await
    }

    pub async fn goods_receipts_get(
        &self,
        token: &str,
    ) -> Result<Vec<GoodsReceiptHeadResponseDto>, ApiError> {
        self.get(&self.goods_receipt_url, token).await
    }

    pub async fn goods_receipt_create(
        &self,
        token: &str,
        goods_receipt: GoodsReceiptHeadCreateDto,
    ) -> Result<GoodsReceiptHeadResponseDto, ApiError> {
        self.post(&self.goods_receipt_url, token, &goods_receipt)
            .await
    }

    pub async fn goods_receipt_confirm(&self, token: &str, id: &Uuid) -> Result<(), ApiError> {
        self.confirm(
            &format!("{}/{}/confirm", &self.goods_receipt_url, id),
            token,
        )
        .await
    }

    pub async fn goods_receipt_delete(&self, token: &str, id: &Uuid) -> Result<(), ApiError> {
        self.delete(&format!("{}/{}", &self.goods_receipt_url, id), token)
            .await
    }

    pub async fn warehouse_stocks_get(
        &self,
        token: &str,
    ) -> Result<Vec<WarehouseStockResponseDto>, ApiError> {
        self.get(&self.warehouse_stock_url, token).await
    }

    pub async fn invoices_get(&self, token: &str) -> Result<Vec<InvoiceHeadResponseDto>, ApiError> {
        self.get(&self.invoice_url, token).await
    }

    pub async fn invoice_create(
        &self,
        token: &str,
        invoice: InvoiceHeadCreateDto,
    ) -> Result<InvoiceHeadResponseDto, ApiError> {
        self.post(&self.invoice_url, token, &invoice).await
    }

    pub async fn invoice_confirm(&self, token: &str, id: &Uuid) -> Result<(), ApiError> {
        self.confirm(&format!("{}/{}/confirm", &self.invoice_url, id), token)
            .await
    }

    pub async fn invoice_delete(&self, token: &str, id: &Uuid) -> Result<(), ApiError> {
        self.delete(&format!("{}/{}", self.invoice_url, id), token)
            .await
    }

    async fn get<T: DeserializeOwned>(&self, url: &str, token: &str) -> Result<T, ApiError> {
        match self
            .client
            .get(url)
            .header(AUTH_HEADER_KEY, create_auth_header(token))
            .send()
            .await
        {
            Ok(res) => {
                if res.status() != StatusCode::OK {
                    return Err(ApiError::InvalidStatus);
                }

                match res.json::<T>().await {
                    Ok(parsed) => Ok(parsed),
                    Err(e) => {
                        tracing::error!("{}. Error parsing get result: {}", LOG_CONTEXT, e);
                        Err(ApiError::ResultParseError)
                    }
                }
            }
            Err(_) => Err(ApiError::General),
        }

        // Ok(response_result.json::<T>().await?)
    }

    async fn post<T: Serialize, U: DeserializeOwned>(
        &self,
        url: &str,
        token: &str,
        payload: &T,
    ) -> Result<U, ApiError> {
        let response_result = self
            .client
            .post(url)
            .header(AUTH_HEADER_KEY, create_auth_header(token))
            .json(payload)
            .send()
            .await;

        match response_result {
            Ok(response) => match response.json::<U>().await {
                Ok(parsed) => Ok(parsed),
                Err(e) => {
                    tracing::error!("{}. Error parsing post result: {}", LOG_CONTEXT, e);
                    Err(ApiError::ResultParseError)
                }
            },
            Err(_) => Err(ApiError::General),
        }
    }

    async fn delete(&self, url: &str, token: &str) -> Result<(), ApiError> {
        match self
            .client
            .delete(url)
            .header(AUTH_HEADER_KEY, create_auth_header(token))
            .send()
            .await
        {
            Ok(response) => {
                if response.status() != StatusCode::NO_CONTENT {
                    tracing::error!(
                        "{}. delete invalid status: {}",
                        LOG_CONTEXT,
                        response.status()
                    );

                    return Err(ApiError::InvalidStatus);
                }

                Ok(())
            }
            Err(e) => {
                tracing::error!("{}. delete error: {}", LOG_CONTEXT, e);
                Err(ApiError::General)
            }
        }
    }

    async fn confirm(&self, url: &str, token: &str) -> Result<(), ApiError> {
        match self
            .client
            .patch(url)
            .header(AUTH_HEADER_KEY, create_auth_header(token))
            .send()
            .await
        {
            Ok(response) => {
                if response.status() != StatusCode::NO_CONTENT {
                    tracing::error!(
                        "{}. confirm invalid status: {}",
                        LOG_CONTEXT,
                        response.status()
                    );
                    return Err(ApiError::InvalidStatus);
                }

                Ok(())
            }
            Err(e) => {
                tracing::error!("{}. confirm error: {}", LOG_CONTEXT, e);
                Err(ApiError::General)
            }
        }
    }
}

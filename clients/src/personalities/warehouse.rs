use std::{sync::Arc, time::Duration};

use rand::{SeedableRng, rngs::StdRng, seq::IndexedRandom};
use rust_decimal::Decimal;

use crate::{
    PASSWORD,
    api_client::ApiClient,
    errors::ApiError,
    helpers::generate_random_string,
    models::{
        goods_receipt_head_create_dto::GoodsReceiptHeadCreateDto,
        goods_receipt_item_create_dto::GoodsReceiptItemCreateDto,
    },
};

const LOG_CONTEXT: &str = "WAREHOUSE";

pub struct Warehouse {
    pub name: String,
    client: Arc<ApiClient>,
    auth_token: Option<String>,
    rng: StdRng,
}

impl Warehouse {
    pub fn new(name: String, client: Arc<ApiClient>) -> Self {
        Self {
            name,
            client,
            auth_token: None,
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    pub async fn run(mut self) {
        let numbers: Vec<u64> = (10..20).collect();

        loop {
            self.run_once().await;

            let seconds = numbers.choose(&mut self.rng).unwrap();
            tokio::time::sleep(Duration::from_secs(*seconds)).await;
        }
    }

    async fn run_once(&mut self) {
        if let Some(token) = self.token_get().await {
            self.create_goods_receipt(&token).await;
            self.handle_goods_receipts(&token).await;
        }
    }

    async fn handle_goods_receipts(&mut self, token: &str) {
        let goods_receipts = match self.client.goods_receipts_get(token).await {
            Ok(res) => res,
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
                return;
            }
            Err(e) => {
                tracing::error!("{}. goods_receipts_get error: {}", LOG_CONTEXT, e);
                return;
            }
        };

        if goods_receipts.len() > 10 {
            let receipt1 = match goods_receipts.choose(&mut self.rng) {
                Some(gr) => gr,
                None => return,
            };

            match self.client.goods_receipt_confirm(token, &receipt1.id).await {
                Ok(_) => {
                    tracing::info!("{}. goods receipt confirmed: {}", LOG_CONTEXT, &receipt1.id)
                }
                Err(ApiError::Unauthorized) => {
                    self.reset_token().await;
                    return;
                }
                Err(e) => {
                    tracing::error!("{}. goods_receipt_confirm error: {}", LOG_CONTEXT, e);
                    return;
                }
            }

            let receipt2 = match goods_receipts.choose(&mut self.rng) {
                Some(gr) => gr,
                None => return,
            };

            if receipt1.id != receipt2.id {
                match self.client.goods_receipt_delete(token, &receipt2.id).await {
                    Ok(_) => {
                        tracing::info!("{}. goods receipt deleted: {}", LOG_CONTEXT, &receipt2.id);
                    }
                    Err(ApiError::Unauthorized) => {
                        self.reset_token().await;
                    }
                    Err(e) => {
                        tracing::error!("{}. goods_receipt_delete error: {}", LOG_CONTEXT, e);
                    }
                }
            }
        }
    }

    async fn create_goods_receipt(&mut self, token: &str) {
        let stocks = match self.client.warehouse_stocks_get(token).await {
            Ok(res) => res,
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
                return;
            }
            Err(e) => {
                tracing::error!("{}. warehouse_stock_get error: {}", LOG_CONTEXT, e);
                return;
            }
        };

        let articles = match self.client.articles_get(token).await {
            Ok(res) => res,
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
                return;
            }
            Err(e) => {
                tracing::error!("{}. create_goods_receipt error: {}", LOG_CONTEXT, e);
                return;
            }
        };

        if articles.len() < 20 {
            return;
        }

        let numbers: Vec<u8> = (1..19).collect();
        let no_of_articles = match numbers.choose(&mut self.rng) {
            Some(res) => res,
            None => return,
        };

        let mut items = vec![];

        for _ in 0..*no_of_articles {
            let article = match articles.choose(&mut self.rng) {
                Some(res) => res,
                None => continue,
            };

            let stock = stocks.iter().find(|i| i.article_id == article.id);

            if stock.is_some_and(|s| s.quantity > Decimal::from(100)) {
                continue;
            }

            if !items
                .iter()
                .any(|i: &GoodsReceiptItemCreateDto| i.article_id == article.id)
            {
                let quantity = match (1..100).collect::<Vec<u16>>().choose(&mut self.rng) {
                    Some(res) => Decimal::from(*res),
                    None => return,
                };

                items.push(GoodsReceiptItemCreateDto {
                    article_id: article.id,
                    ordinal: items.len() as i32,
                    quantity,
                });
            }
        }

        if items.is_empty() {
            tracing::info!("{}. No items. Skipping goods receipt creation", LOG_CONTEXT);
            return;
        }

        let goods_receipt = GoodsReceiptHeadCreateDto {
            supplier_name: generate_random_string(20),
            items,
        };

        match self.client.goods_receipt_create(token, goods_receipt).await {
            Ok(res) => tracing::info!(
                "{}. Goods receipt created for supplier: {}",
                LOG_CONTEXT,
                res.supplier_name
            ),
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
            }
            Err(e) => tracing::error!("{}. Error creating goods receipt: {}", LOG_CONTEXT, e),
        }
    }

    async fn reset_token(&mut self) {
        tracing::info!("{}. Reseting API token", LOG_CONTEXT);

        self.auth_token = None;
        self.token_get().await;
    }

    async fn token_get(&mut self) -> Option<String> {
        if self.auth_token.is_none() {
            self.auth_token = self.token_fetch().await;
        }

        self.auth_token.clone()
    }

    async fn token_fetch(&self) -> Option<String> {
        match self
            .client
            .login(self.name.clone(), PASSWORD.to_string())
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

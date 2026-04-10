use std::{sync::Arc, time::Duration};

use rand::{SeedableRng, rngs::StdRng, seq::IndexedRandom};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    PASSWORD,
    api_client::ApiClient,
    errors::ApiError,
    helpers::generate_random_string,
    models::{
        invoice_head_create_dto::InvoiceHeadCreateDto,
        invoice_item_create_dto::InvoiceItemCreateDto,
    },
};

const LOG_CONTEXT: &str = "SALES";

pub struct Sales {
    pub name: String,
    client: Arc<ApiClient>,
    auth_token: Option<String>,
    rng: StdRng,
}

impl Sales {
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
            self.create_invoice(&token).await;
            self.handle_invoices(&token).await;
        }
    }

    async fn handle_invoices(&mut self, token: &str) {
        let invoices = match self.client.invoices_get(token).await {
            Ok(i) => i,
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
                return;
            }
            Err(e) => {
                tracing::error!("{}. invoices_get error: {}", LOG_CONTEXT, e);
                return;
            }
        };

        if invoices.len() > 5 {
            let invoice1 = match invoices.choose(&mut self.rng) {
                Some(i) => i,
                None => return,
            };

            self.confirm_invoice(token, &invoice1.id).await;

            let invoice2 = match invoices.choose(&mut self.rng) {
                Some(i) => i,
                None => return,
            };

            if invoice2.id != invoice1.id {
                self.confirm_invoice(token, &invoice2.id).await;
            }

            let invoice3 = match invoices.choose(&mut self.rng) {
                Some(i) => i,
                None => return,
            };

            if invoice3.id != invoice1.id && invoice3.id != invoice2.id {
                self.delete_invoice(token, &invoice3.id).await;
            }
        }
    }

    async fn confirm_invoice(&mut self, token: &str, id: &Uuid) {
        match self.client.invoice_confirm(token, id).await {
            Ok(_) => {
                tracing::info!("{}. Invoice {} confirmed", LOG_CONTEXT, id);
            }
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
            }
            Err(e) => {
                tracing::error!(
                    "{}. invoice {} confirm error: {}. Gonna try to delete it",
                    LOG_CONTEXT,
                    id,
                    e
                );

                self.delete_invoice(token, id).await;
            }
        }
    }

    async fn delete_invoice(&mut self, token: &str, id: &Uuid) {
        match self.client.invoice_delete(token, id).await {
            Ok(_) => {
                tracing::info!("{}. Invoice {} successfully deleted", LOG_CONTEXT, id);
            }
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
            }
            Err(e) => {
                tracing::error!("{}. Error deleting invoice {}: {}", LOG_CONTEXT, id, e);
            }
        }
    }

    async fn create_invoice(&mut self, token: &str) {
        let invoices = match self.client.invoices_get(token).await {
            Ok(res) => res,
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
                return;
            }
            Err(e) => {
                tracing::error!("{}. Error getting invoices: {}", LOG_CONTEXT, e);
                return;
            }
        };

        if invoices.len() > 100 {
            return;
        }

        let stocks = match self.client.warehouse_stocks_get(token).await {
            Ok(s) => s,
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
                return;
            }
            Err(e) => {
                tracing::error!("{}. warehouse_stocks_get error: {}", LOG_CONTEXT, e);
                return;
            }
        };

        if stocks.len() < 5 {
            return;
        }

        let numbers: Vec<u8> = (1..8).collect();
        let no_of_items = match numbers.choose(&mut self.rng) {
            Some(n) => n,
            None => return,
        };

        let mut items: Vec<InvoiceItemCreateDto> = vec![];

        for _ in 0..*no_of_items {
            let stock = match stocks.choose(&mut self.rng) {
                Some(s) => s,
                None => continue,
            };

            if !items.iter().any(|item| item.article_id == stock.article_id) {
                let quantity = match (1..15).collect::<Vec<u8>>().choose(&mut self.rng) {
                    Some(q) => Decimal::from(*q),
                    None => continue,
                };

                items.push(InvoiceItemCreateDto {
                    article_id: stock.article_id,
                    ordinal: items.len() as i32,
                    quantity,
                });
            }
        }

        if items.is_empty() {
            tracing::info!("{}. No items. Skipping invoice creation", LOG_CONTEXT);
            return;
        }

        let invoice = InvoiceHeadCreateDto {
            customer_name: generate_random_string(16),
            items,
        };

        match self.client.invoice_create(token, invoice).await {
            Ok(res) => {
                tracing::info!(
                    "{}. Invoice created for customer: {}",
                    LOG_CONTEXT,
                    res.customer_name
                );
            }
            Err(ApiError::Unauthorized) => {
                self.reset_token().await;
            }
            Err(e) => {
                tracing::error!("{}. Error creating invoice: {}", LOG_CONTEXT, e);
            }
        }
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
            Ok(username) => Some(username),
            Err(e) => {
                tracing::error!("{}. client.login error: {}", LOG_CONTEXT, e);
                None
            }
        }
    }

    async fn reset_token(&mut self) {
        tracing::info!("{}. Reseting API token", LOG_CONTEXT);

        self.auth_token = None;
        self.token_get().await;
    }
}

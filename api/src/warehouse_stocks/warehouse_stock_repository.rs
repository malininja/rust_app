use rust_decimal::Decimal;
use sqlx::{Error, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::warehouse_stocks::warehouse_stock_model::WarehouseStockModel;

pub struct PgWarehouseStockRepository {
    pub pool: PgPool,
}

impl PgWarehouseStockRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<WarehouseStockModel>, Error> {
        sqlx::query_as!(
            WarehouseStockModel,
            "
            SELECT ws.id, ws.article_id, ws.quantity, ws.created_at, ws.updated_at
            FROM warehouse_stocks ws
            JOIN articles a ON ws.article_id = a.id
            WHERE a.deleted_at IS NULL
            "
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_article_id(
        &self,
        article_id: Uuid,
    ) -> Result<Option<WarehouseStockModel>, Error> {
        sqlx::query_as!(
            WarehouseStockModel,
            "
            SELECT ws.id, ws.article_id, ws.quantity, ws.created_at, ws.updated_at
            FROM warehouse_stocks ws
            JOIN articles a ON ws.article_id = a.id
            WHERE ws.article_id = $1
              AND a.deleted_at IS NULL
            ",
            article_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}

pub async fn update_quantity(
    tx: &mut Transaction<'_, Postgres>,
    article_quantities: Vec<(Uuid, Decimal)>,
) -> Result<(), Error> {
    let article_ids: Vec<Uuid> = article_quantities
        .iter()
        .map(|(article_id, _)| *article_id)
        .collect();
    let quantities: Vec<Decimal> = article_quantities
        .iter()
        .map(|(_, quantity)| *quantity)
        .collect();

    if !quantities.is_empty() && quantities[0] < Decimal::from(0) {
        // When using INSERT ON CONFLICT UPDATE with negative quantities check constraint fails on insert, even though article exists
        sqlx::query!(
            "
            UPDATE warehouse_stocks
              SET quantity = warehouse_stocks.quantity + t.quantity
              FROM UNNEST($1::UUID[], $2::NUMERIC[]) as t(article_id, quantity)
              WHERE warehouse_stocks.article_id = t.article_id
            ",
            &article_ids,
            &quantities
        )
        .execute(tx.as_mut())
        .await?;
    } else {
        sqlx::query!(
            "
            INSERT INTO warehouse_stocks (article_id, quantity) 
            SELECT * FROM UNNEST($1::UUID[], $2::NUMERIC[])
            ON CONFLICT (article_id)
            DO UPDATE SET quantity = warehouse_stocks.quantity + EXCLUDED.quantity
            ",
            &article_ids,
            &quantities,
        )
        .execute(tx.as_mut())
        .await?;
    }

    Ok(())
}

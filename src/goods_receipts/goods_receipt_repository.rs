use rust_decimal::Decimal;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::goods_receipts::{
    dtos::goods_receipt_item_create_dto::GoodsReceiptItemCreateDto,
    goods_receipt_head_model::GoodsReceiptHeadModel,
    goods_receipt_item_model::GoodsReceiptItemModel,
};

pub struct PgGoodsReceiptRepository {
    pub pool: PgPool,
}

impl PgGoodsReceiptRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_unconfirmed(&self) -> Result<Vec<GoodsReceiptHeadModel>, Error> {
        sqlx::query_as!(
            GoodsReceiptHeadModel,
            "
          SELECT id, supplier_name, confirmed, created_at, updated_at, deleted_at
          FROM goods_receipt_heads
          WHERE confirmed=false
            AND deleted_at IS NULL
          "
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<(GoodsReceiptHeadModel, Vec<GoodsReceiptItemModel>)>, Error> {
        let head_option = sqlx::query_as!(
            GoodsReceiptHeadModel,
            "SELECT * FROM goods_receipt_heads WHERE id=$1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(head) = head_option {
            let items = self.get_items(id).await?;
            return Ok(Some((head, items)));
        }

        Ok(None)
    }

    pub async fn create(
        &self,
        supplier_name: String,
        items: Vec<GoodsReceiptItemCreateDto>,
    ) -> Result<(GoodsReceiptHeadModel, Vec<GoodsReceiptItemModel>), Error> {
        let article_ids: Vec<Uuid> = items.iter().map(|i| i.article_id).collect();
        let ordinals: Vec<i32> = items.iter().map(|i| i.ordinal).collect();
        let quantities: Vec<Decimal> = items.iter().map(|i| i.quantity).collect();

        let create_result = sqlx::query!(
            "
            WITH head AS (
              INSERT INTO goods_receipt_heads (supplier_name) VALUES($1)
              RETURNING id
            ),
            items AS (
            INSERT INTO goods_receipt_items (goods_receipt_head_id, article_id, ordinal, quantity)
              SELECT (SELECT id FROM head), a, o, q FROM UNNEST($2::UUID[], $3::INT[], $4::NUMERIC[]) as t(a, o, q)
            )
            SELECT id FROM head
            ",
            supplier_name,
            &article_ids,
            &ordinals,
            &quantities,
        )
        .fetch_one(&self.pool)
        .await?;

        let head = self.get_head(create_result.id).await?;

        let items = self.get_items(create_result.id).await?;

        Ok((head, items))
    }

    pub async fn update(
        &self,
        id: Uuid,
        supplier_name: Option<String>,
        confirmed: Option<bool>,
        items: Option<Vec<GoodsReceiptItemCreateDto>>,
    ) -> Result<(GoodsReceiptHeadModel, Vec<GoodsReceiptItemModel>), Error> {
        let mut tx = self.pool.begin().await?;

        let _ = sqlx::query!(
            "
            UPDATE goods_receipt_heads SET
            supplier_name = COALESCE($2::TEXT, supplier_name),
            confirmed = COALESCE($3::BOOLEAN, confirmed)
            WHERE id = $1
            ",
            id,
            supplier_name,
            confirmed
        )
        .execute(&mut *tx)
        .await?;

        if let Some(unwrapped) = items {
            let article_ids: Vec<Uuid> = unwrapped.iter().map(|i| i.article_id).collect();
            let ordinals: Vec<i32> = unwrapped.iter().map(|i| i.ordinal).collect();
            let quantities: Vec<Decimal> = unwrapped.iter().map(|i| i.quantity).collect();

            let _ = sqlx::query!(
                "
                WITH _ AS (DELETE FROM goods_receipt_items WHERE goods_receipt_head_id = $1)
                INSERT INTO goods_receipt_items (goods_receipt_head_id, article_id, ordinal, quantity)
                  SELECT (SELECT $1), a, o, q FROM UNNEST($2::UUID[], $3::INT[], $4::NUMERIC[]) AS t(a, o, q)
                ",
                id,
                &article_ids,
                &ordinals,
                &quantities,
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        let head = self.get_head(id).await?;

        let items = self.get_items(id).await?;

        Ok((head, items))
    }

    pub async fn soft_delete(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE goods_receipt_heads SET deleted_at = NOW() WHERE id=$1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_head(&self, id: Uuid) -> Result<GoodsReceiptHeadModel, Error> {
        sqlx::query_as!(
            GoodsReceiptHeadModel,
            "SELECT * FROM goods_receipt_heads WHERE id=$1",
            id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn get_items(&self, head_id: Uuid) -> Result<Vec<GoodsReceiptItemModel>, Error> {
        sqlx::query_as!(
            GoodsReceiptItemModel,
            "SELECT * FROM goods_receipt_items WHERE goods_receipt_head_id=$1",
            head_id
        )
        .fetch_all(&self.pool)
        .await
    }
}

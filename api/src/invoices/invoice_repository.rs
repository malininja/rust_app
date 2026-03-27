use rust_decimal::Decimal;
use sqlx::{Error, PgExecutor, PgPool};
use uuid::Uuid;

use crate::{
    invoices::{
        dtos::invoice_item_create_dto::InvoiceItemCreateDto, invoice_head_model::InvoiceHeadModel,
        invoice_item_model::InvoiceItemModel,
    },
    warehouse_stocks::warehouse_stock_repository::update_quantity,
};

pub struct PgInvoiceRepository {
    pub pool: PgPool,
}

impl PgInvoiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_unconfirmed(&self) -> Result<Vec<InvoiceHeadModel>, Error> {
        sqlx::query_as!(
            InvoiceHeadModel,
            "
          SELECT id, customer_name, confirmed, created_at, updated_at, deleted_at
          FROM invoice_heads
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
    ) -> Result<Option<(InvoiceHeadModel, Vec<InvoiceItemModel>)>, Error> {
        let head_option = get_by_id(&self.pool, id).await?;

        if let Some(head) = head_option {
            let items = get_items(&self.pool, id).await?;
            return Ok(Some((head, items)));
        }

        Ok(None)
    }

    pub async fn create(
        &self,
        customer_name: String,
        items: Vec<InvoiceItemCreateDto>,
    ) -> Result<(InvoiceHeadModel, Vec<InvoiceItemModel>), Error> {
        let article_ids: Vec<Uuid> = items.iter().map(|i| i.article_id).collect();
        let ordinals: Vec<i32> = items.iter().map(|i| i.ordinal).collect();
        let quantities: Vec<Decimal> = items.iter().map(|i| i.quantity).collect();

        let create_result = sqlx::query!(
            "
            WITH head AS (
              INSERT INTO invoice_heads (customer_name) VALUES($1)
              RETURNING id
            ),
            items AS (
            INSERT INTO invoice_items (invoice_head_id, article_id, ordinal, quantity)
              SELECT (SELECT id FROM head), a, o, q FROM UNNEST($2::UUID[], $3::INT[], $4::NUMERIC[]) as t(a, o, q)
            )
            SELECT id FROM head
            ",
            customer_name,
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

    pub async fn confirm(&self, id: Uuid) -> Result<InvoiceHeadModel, Error> {
        let mut tx = self.pool.begin().await?;

        let head = sqlx::query_as!(
            InvoiceHeadModel,
            "
            UPDATE invoice_heads
              SET confirmed = true
            WHERE id = $1
              AND confirmed = false
            RETURNING id, customer_name, confirmed, created_at, updated_at, deleted_at   
            ",
            id
        )
        .fetch_one(tx.as_mut())
        .await?;

        let items = get_items(tx.as_mut(), id).await?;

        let article_quantities: Vec<(Uuid, Decimal)> =
            items.iter().map(|i| (i.article_id, -i.quantity)).collect();

        update_quantity(&mut tx, article_quantities).await?;

        tx.commit().await?;

        Ok(head)
    }

    pub async fn update(
        &self,
        id: Uuid,
        customer_name: Option<String>,
        items: Option<Vec<InvoiceItemCreateDto>>,
    ) -> Result<Option<(InvoiceHeadModel, Vec<InvoiceItemModel>)>, Error> {
        let mut tx = self.pool.begin().await?;

        let updated = sqlx::query!(
            "
            UPDATE invoice_heads SET
            customer_name = COALESCE($2::TEXT, customer_name)
            WHERE id = $1
              AND deleted_at IS NULL
            RETURNING id
            ",
            id,
            customer_name
        )
        .fetch_optional(&mut *tx)
        .await?;

        if updated.is_none() {
            return Ok(None);
        }

        if let Some(unwrapped) = items {
            let article_ids: Vec<Uuid> = unwrapped.iter().map(|i| i.article_id).collect();
            let ordinals: Vec<i32> = unwrapped.iter().map(|i| i.ordinal).collect();
            let quantities: Vec<Decimal> = unwrapped.iter().map(|i| i.quantity).collect();

            let _ = sqlx::query!("DELETE FROM invoice_items WHERE invoice_head_id = $1", id)
                .execute(&mut *tx)
                .await;

            let _ = sqlx::query!(
                "
                INSERT INTO invoice_items (invoice_head_id, article_id, ordinal, quantity)
                  SELECT (SELECT $1::UUID), a, o, q FROM UNNEST($2::UUID[], $3::INT[], $4::NUMERIC[]) AS t(a, o, q)
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

        Ok(Some((head, items)))
    }

    pub async fn soft_delete(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE invoice_heads SET deleted_at = NOW() WHERE id=$1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_head(&self, id: Uuid) -> Result<InvoiceHeadModel, Error> {
        sqlx::query_as!(
            InvoiceHeadModel,
            "SELECT * FROM invoice_heads WHERE id=$1",
            id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn get_items(&self, head_id: Uuid) -> Result<Vec<InvoiceItemModel>, Error> {
        sqlx::query_as!(
            InvoiceItemModel,
            "SELECT * FROM invoice_items WHERE invoice_head_id=$1",
            head_id
        )
        .fetch_all(&self.pool)
        .await
    }
}

async fn get_by_id<'e, E: PgExecutor<'e>>(
    executor: E,
    id: Uuid,
) -> Result<Option<InvoiceHeadModel>, Error> {
    sqlx::query_as!(
        InvoiceHeadModel,
        "SELECT * FROM invoice_heads WHERE id=$1",
        id
    )
    .fetch_optional(executor)
    .await
}

async fn get_items<'e, E: PgExecutor<'e>>(
    executor: E,
    head_id: Uuid,
) -> Result<Vec<InvoiceItemModel>, Error> {
    sqlx::query_as!(
        InvoiceItemModel,
        "SELECT * FROM invoice_items WHERE invoice_head_id=$1",
        head_id
    )
    .fetch_all(executor)
    .await
}

use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::articles::article_model::{ArticleModel, UnitOfMeasure};

#[async_trait]
pub trait ArticleRepository {
    async fn get_all_articles(&self) -> Result<Vec<ArticleModel>, Error>;

    async fn get_article_by_id(&self, id: Uuid) -> Result<Option<ArticleModel>, Error>;

    async fn create_article(
        &self,
        name: String,
        unit_of_measure: UnitOfMeasure,
    ) -> Result<ArticleModel, Error>;

    async fn update_article(
        &self,
        id: Uuid,
        name: Option<String>,
        unit_of_measure: Option<UnitOfMeasure>,
    ) -> Result<Option<ArticleModel>, Error>;

    async fn soft_delete_article(&self, id: Uuid) -> Result<Option<ArticleModel>, Error>;
}

pub struct PgArticleRepository {
    pool: PgPool,
}

impl PgArticleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticleRepository for PgArticleRepository {
    async fn get_all_articles(&self) -> Result<Vec<ArticleModel>, Error> {
        Ok(sqlx::query_as!(
            ArticleModel,
            r#"
            SELECT id, name, unit_of_measure AS "unit_of_measure: UnitOfMeasure", created_at, updated_at, deleted_at
            FROM articles
            WHERE deleted_at is NULL
            "#,
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn get_article_by_id(&self, id: Uuid) -> Result<Option<ArticleModel>, Error> {
        Ok(sqlx::query_as!(
            ArticleModel,
            r#"
            SELECT id, name, unit_of_measure AS "unit_of_measure: UnitOfMeasure", created_at, updated_at, deleted_at
            FROM articles
            WHERE id=$1
              AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn create_article(
        &self,
        name: String,
        unit_of_measure: UnitOfMeasure,
    ) -> Result<ArticleModel, Error> {
        Ok(sqlx::query_as!(
            ArticleModel,
            r#"
            INSERT INTO articles (name, unit_of_measure) VALUES($1, $2::unit_of_measure)
            RETURNING id, name, unit_of_measure AS "unit_of_measure: UnitOfMeasure", created_at, updated_at, deleted_at
            "#,
            name,
            unit_of_measure as _
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn update_article(
        &self,
        id: Uuid,
        name: Option<String>,
        unit_of_measure: Option<UnitOfMeasure>,
    ) -> Result<Option<ArticleModel>, Error> {
        Ok(sqlx::query_as!(
            ArticleModel,
            r#"
            UPDATE articles SET
              name = COALESCE($2::TEXT, name),
              unit_of_measure = COALESCE($3::unit_of_measure, unit_of_measure)
            WHERE id=$1 AND deleted_at IS NULL
            RETURNING id, name, unit_of_measure AS "unit_of_measure: UnitOfMeasure", created_at, updated_at, deleted_at
            "#,
            id,
            name,
            unit_of_measure as _
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn soft_delete_article(&self, id: Uuid) -> Result<Option<ArticleModel>, Error> {
        Ok(sqlx::query_as!(
            ArticleModel,
            r#"
            UPDATE articles SET deleted_at = NOW() WHERE id=$1 AND deleted_at IS NULL
            RETURNING id, name, unit_of_measure AS "unit_of_measure: UnitOfMeasure", created_at, updated_at, deleted_at
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }
}

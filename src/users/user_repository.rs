use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::users::user_model::UserModel;

#[async_trait]
pub trait UserRepository {
    async fn get_all_users(&self) -> Result<Vec<UserModel>, Error>;

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserModel>, Error>;

    async fn get_user_by_username(&self, username: String) -> Result<Option<UserModel>, Error>;

    async fn create_user(
        &self,
        role_id: i32,
        username: String,
        hashed_password: String,
    ) -> Result<UserModel, Error>;

    async fn update_user(
        &self,
        id: Uuid,
        role_id: Option<i32>,
        username: Option<String>,
        hashed_password: Option<String>,
    ) -> Result<Option<UserModel>, Error>;

    async fn soft_delete_user(&self, id: Uuid) -> Result<Option<UserModel>, Error>;

    async fn soft_undelete_user(&self, id: Uuid) -> Result<Option<UserModel>, Error>;
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn get_all_users(&self) -> Result<Vec<UserModel>, Error> {
        Ok(sqlx::query_as!(
            UserModel,
            "
            SELECT id, role_id, username, password, created_at, updated_at, deleted_at 
            FROM users
            WHERE deleted_at IS NULL
            "
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserModel>, Error> {
        Ok(sqlx::query_as!(
            UserModel,
            "
            SELECT id, role_id, username, password, created_at, updated_at, deleted_at 
            FROM users 
            WHERE id=$1
              AND deleted_at IS NULL
            ",
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn get_user_by_username(&self, username: String) -> Result<Option<UserModel>, Error> {
        Ok(sqlx::query_as!(
            UserModel,
            "
            SELECT id, role_id, username, password, created_at, updated_at, deleted_at 
            FROM users 
            WHERE username=$1
              AND deleted_at IS NULL
            ",
            username
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn create_user(
        &self,
        role_id: i32,
        username: String,
        hashed_password: String,
    ) -> Result<UserModel, Error> {
        Ok(sqlx::query_as!(
            UserModel,
            "
            INSERT INTO users (role_id, username, password) 
            VALUES ($1, $2, $3)
            RETURNING id, role_id, username, password, created_at, updated_at, deleted_at
            ",
            role_id,
            username,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn update_user(
        &self,
        id: Uuid,
        role_id: Option<i32>,
        username: Option<String>,
        hashed_password: Option<String>,
    ) -> Result<Option<UserModel>, Error> {
        Ok(sqlx::query_as!(
            UserModel,
            "
            UPDATE users SET 
              role_id  = COALESCE($1::INTEGER, role_id),
              username = COALESCE($2::VARCHAR, username),
              password = COALESCE($3::VARCHAR, password)
            WHERE id = $4 AND deleted_at IS NULL
            RETURNING id, role_id, username, password, created_at, updated_at, deleted_at
            ",
            role_id,
            username,
            hashed_password,
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn soft_delete_user(&self, id: Uuid) -> Result<Option<UserModel>, Error> {
        Ok(sqlx::query_as!(
            UserModel,
            "
            UPDATE users SET deleted_at = NOW()
            WHERE id=$1
              AND deleted_at IS NULL
            RETURNING id, role_id, username, password, created_at, updated_at, deleted_at
            ",
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn soft_undelete_user(&self, id: Uuid) -> Result<Option<UserModel>, Error> {
        Ok(sqlx::query_as!(
            UserModel,
            "
            UPDATE users SET deleted_at = NULL 
            WHERE id=$1
            AND deleted_at IS NOT NULL
            RETURNING id, role_id, username, password, created_at, updated_at, deleted_at
            ",
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }
}

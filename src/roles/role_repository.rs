use crate::roles::role_model::RoleModel;
use async_trait::async_trait;
use sqlx::{Error, PgPool};

#[async_trait]
pub trait RoleRepository {
    async fn get_all_roles(&self) -> Result<Vec<RoleModel>, Error>;
}

pub struct PgRoleRepository {
    pool: PgPool,
}

impl PgRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for PgRoleRepository {
    async fn get_all_roles(&self) -> Result<Vec<RoleModel>, Error> {
        Ok(sqlx::query_as!(
            RoleModel,
            "SELECT id, code, name, created_at, updated_at FROM roles"
        )
        .fetch_all(&self.pool)
        .await?)
    }
}

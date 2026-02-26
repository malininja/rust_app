use sqlx::{Error, PgPool};

use crate::roles::role_model::RoleModel;

pub async fn get_all_roles(pool: &PgPool) -> Result<Vec<RoleModel>, Error> {
    sqlx::query_as!(RoleModel, "SELECT id, code, name, created_at, updated_at FROM roles")
        .fetch_all(pool)
        .await
}

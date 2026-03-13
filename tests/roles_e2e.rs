mod common;

use rust_app::roles::role_model::RoleModel;
use sqlx::PgPool;

use crate::common::{TestApp, create_test_app};

#[sqlx::test]
async fn get_all_roles(pool: PgPool) {
    let TestApp { base_url, port } = create_test_app(pool).await;

    let roles_url = format!("http://{base_url}:{port}/roles");
    let body = reqwest::get(roles_url)
        .await
        .unwrap()
        .json::<Vec<RoleModel>>()
        .await
        .unwrap();

    assert_eq!(body.len(), 2);
    assert!(body.iter().any(|x| x.code == "A"));
    assert!(body.iter().any(|x| x.code == "U"));
}

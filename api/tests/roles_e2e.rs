mod common;

use rust_app::{AppState, roles::role_model::RoleModel};
use sqlx::PgPool;

use crate::common::{TEST_JWT_SECRET, create_test_app, get_admin_token};

#[sqlx::test]
async fn get_all_roles(pool: PgPool) {
    let test_app = create_test_app(AppState {
        pool,
        jwt_secret: TEST_JWT_SECRET.to_string(),
    })
    .await;

    let token = get_admin_token(&test_app).await;

    let reqwest_client = reqwest::Client::new();

    let roles_url = format!("http://{}:{}/roles", test_app.base_url, test_app.port);
    let body = reqwest_client
        .get(roles_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json::<Vec<RoleModel>>()
        .await
        .unwrap();

    assert_eq!(body.len(), 2);
    assert!(body.iter().any(|x| x.code == "ADMIN"));
    assert!(body.iter().any(|x| x.code == "USER"));
}

use std::time::Duration;

use rust_app::{create_app, roles::role_model::RoleModel};
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn get_all_roles() {
    let base_url = "127.0.0.1";
    let _ = dotenvy::dotenv();
    let cnn_string = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&cnn_string)
        .await
        .expect("Can't connect database.");

    let app = create_app(pool);
    let listener = tokio::net::TcpListener::bind(format!("{base_url}:0"))
        .await
        .unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(axum::serve(listener, app).into_future());

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

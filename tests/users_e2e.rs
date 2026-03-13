mod common;

use reqwest::StatusCode;
use rust_app::AppState;
use rust_app::users::dtos::user_create_dto::UserCreateDto;
use rust_app::users::dtos::user_update_dto::UserUpdateDto;
use rust_app::{roles::role_model::RoleModel, users::dtos::user_response_dto::UserResponseDto};
use sqlx::PgPool;

use crate::common::{TEST_JWT_SECRET, TestApp, create_test_app};

#[sqlx::test]
async fn test_users_router(pool: PgPool) {
    let TestApp { base_url, port } = create_test_app(AppState {
        pool,
        jwt_secret: TEST_JWT_SECRET.to_string(),
    })
    .await;

    let reqwest_client = reqwest::Client::new();

    let users_url = format!("http://{base_url}:{port}/users");

    //############# CREATE
    let roles_url = format!("http://{base_url}:{port}/roles");
    let roles = reqwest::get(roles_url)
        .await
        .unwrap()
        .json::<Vec<RoleModel>>()
        .await
        .unwrap();

    let admin_role = roles.iter().find(|r| r.code == "ADMIN").unwrap();

    let user_create_dto = UserCreateDto {
        role_id: admin_role.id,
        username: "pero adminić".to_string(),
        password: "superstrongpass".to_string(),
    };

    let created_user_response = reqwest_client
        .post(&users_url)
        .json(&user_create_dto)
        .send()
        .await
        .unwrap();

    let status = created_user_response.status();
    let created_user = created_user_response
        .json::<UserResponseDto>()
        .await
        .unwrap();

    assert_eq!(StatusCode::CREATED, status);
    assert_eq!(user_create_dto.role_id, created_user.role_id);
    assert_eq!(user_create_dto.username, created_user.username);

    //############# GET ALL
    let fetched_users = reqwest_client
        .get(&users_url)
        .send()
        .await
        .unwrap()
        .json::<Vec<UserResponseDto>>()
        .await
        .unwrap();

    assert!(fetched_users.iter().any(|u| u.id == created_user.id));

    //############# GET BY ID
    let fetched_user = reqwest_client
        .get(format!("{}/{}", users_url, created_user.id))
        .send()
        .await
        .unwrap()
        .json::<UserResponseDto>()
        .await
        .unwrap();

    assert_eq!(created_user, fetched_user);

    //############# UPDATE
    let user_update_dto = UserUpdateDto {
        role_id: None,
        username: Some("drugo ime".to_string()),
        password: None,
    };

    let _updated_user = reqwest_client
        .patch(format!("{}/{}", users_url, created_user.id))
        .json(&user_update_dto)
        .send()
        .await
        .unwrap()
        .json::<UserResponseDto>()
        .await
        .unwrap();

    let fetched_updated_user = reqwest_client
        .get(format!("{}/{}", users_url, created_user.id))
        .send()
        .await
        .unwrap()
        .json::<UserResponseDto>()
        .await
        .unwrap();

    assert_ne!(created_user, fetched_updated_user);
    assert_eq!("drugo ime".to_string(), fetched_updated_user.username);

    //############# DELETE
    let user_update_dto = UserUpdateDto {
        role_id: None,
        username: Some("drugo ime".to_string()),
        password: None,
    };

    let delete_response = reqwest_client
        .delete(format!("{}/{}", users_url, created_user.id))
        .json(&user_update_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::NO_CONTENT, delete_response.status());

    let fetch_deleted_response = reqwest_client
        .get(format!("{}/{}", users_url, created_user.id))
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::NOT_FOUND, fetch_deleted_response.status());

    //############# UNDELETE
    let undelete_user_response = reqwest_client
        .patch(format!("{}/{}/undelete", users_url, created_user.id))
        .json(&user_update_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::NO_CONTENT, undelete_user_response.status());

    let fetched_undeleted_user = reqwest_client
        .get(format!("{}/{}", users_url, created_user.id))
        .send()
        .await
        .unwrap()
        .json::<UserResponseDto>()
        .await
        .unwrap();

    assert_eq!(fetched_updated_user, fetched_undeleted_user);
}

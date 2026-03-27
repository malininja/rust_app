use chrono::Utc;
use uuid::Uuid;

use crate::users::{
    dtos::{
        user_create_dto::UserCreateDto, user_response_dto::UserResponseDto,
        user_update_dto::UserUpdateDto,
    },
    tests::user_mock_repository::UserMockRepo,
    user_errors::UserError,
    user_model::UserModel,
    user_service,
};

fn get_admin_user() -> UserModel {
    UserModel {
        id: Uuid::new_v4(),
        role_id: 1,
        username: "admin".to_string(),
        password: "some password".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    }
}

fn get_regular_user() -> UserModel {
    UserModel {
        id: Uuid::new_v4(),
        role_id: 2,
        username: "user".to_string(),
        password: "some password".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    }
}

//################### get_all_users

#[tokio::test]
async fn get_all_users_success() {
    let user_models_mock = vec![get_admin_user(), get_regular_user()];
    let expected: Vec<UserResponseDto> = user_models_mock
        .clone()
        .into_iter()
        .map(|um| um.into())
        .collect();

    let mut repo = UserMockRepo::new();
    repo.get_all_result = Some(Ok(user_models_mock));

    let fetched_data = user_service::get_all_users(&repo).await.unwrap();

    assert_eq!(fetched_data, expected);
}

#[tokio::test]
async fn get_all_users_error() {
    let mut repo = UserMockRepo::new();
    repo.get_all_result = Some(Err(()));

    let fetched = user_service::get_all_users(&repo).await.err().unwrap();

    assert_eq!(fetched, UserError::GetUsersError);
}

//################### get_user_by_id

#[tokio::test]
async fn get_user_by_id_success() {
    let user_model_mock = get_admin_user();
    let expected = UserResponseDto::from(user_model_mock.clone());

    let mut repo = UserMockRepo::new();
    repo.get_by_id_result = Some(Ok(user_model_mock));

    let id = Uuid::new_v4();

    let fetched_data = user_service::get_user_by_id(&repo, id).await.unwrap();

    let repo_user_id = (*repo.captured_get_by_id_args.lock().unwrap()).unwrap();

    assert_eq!(repo_user_id, id);
    assert_eq!(fetched_data, expected);
}

#[tokio::test]
async fn get_user_by_id_error() {
    let mut repo = UserMockRepo::new();
    repo.get_by_id_result = Some(Err(()));

    let fetched = user_service::get_user_by_id(&repo, Uuid::new_v4())
        .await
        .err()
        .unwrap();

    assert_eq!(fetched, UserError::GetUserByIdError);
}

#[tokio::test]
async fn get_user_by_id_does_not_exist() {
    let repo = UserMockRepo::new();

    let fetched = user_service::get_user_by_id(&repo, Uuid::new_v4())
        .await
        .err()
        .unwrap();

    assert_eq!(fetched, UserError::UserNotFound);
}

//################### create user

#[tokio::test]
async fn create_user_success() {
    let user = get_regular_user();

    let mut repo = UserMockRepo::new();
    repo.create_user_result = Some(Ok(user.clone()));

    let user_create_dto = UserCreateDto {
        role_id: 1,
        username: "some username".to_string(),
        password: "some password".to_string(),
    };

    let created = user_service::create_user(&repo, user_create_dto.clone())
        .await
        .unwrap();

    let params = repo.captured_create_args.lock().unwrap().take().unwrap();

    assert_eq!(user_create_dto.role_id, params.0);
    assert_eq!(user_create_dto.username, params.1);
    assert_ne!(0, params.2.len());
    assert_ne!(user_create_dto.password, params.2);
    assert_eq!(UserResponseDto::from(user), created)
}

#[tokio::test]
async fn create_user_error() {
    let mut repo = UserMockRepo::new();
    repo.create_user_result = Some(Err(()));

    let user_create_dto = UserCreateDto {
        role_id: 1,
        username: "some username".to_string(),
        password: "some password".to_string(),
    };

    let created = user_service::create_user(&repo, user_create_dto)
        .await
        .unwrap_err();

    assert_eq!(UserError::CreateUserError, created);
}

//################### update user

#[tokio::test]
async fn update_user_success() {
    let id = Uuid::new_v4();

    let user_update_dto = UserUpdateDto {
        role_id: Some(1),
        username: Some("some user".to_string()),
        password: Some("some password".to_string()),
    };

    let user_model = get_regular_user();
    let expected = UserResponseDto::from(user_model.clone());

    let mut repo = UserMockRepo::new();
    repo.update_user_result = Some(Ok(user_model));

    let updated = user_service::update_user(&repo, id, user_update_dto.clone())
        .await
        .unwrap();

    let params = repo.captured_update_args.lock().unwrap().take().unwrap();

    assert_eq!(id, params.0);
    assert_eq!(user_update_dto.role_id, params.1);
    assert_eq!(user_update_dto.username.unwrap(), params.2.unwrap());
    assert_ne!(user_update_dto.password.unwrap(), params.3.unwrap());
    assert_eq!(expected, updated);
}

#[tokio::test]
async fn update_user_does_not_exist() {
    let user_update_dto = UserUpdateDto {
        role_id: None,
        username: None,
        password: None,
    };

    let repo = UserMockRepo::new();

    let updated = user_service::update_user(&repo, Uuid::new_v4(), user_update_dto)
        .await
        .unwrap_err();

    assert_eq!(UserError::UserNotFound, updated);
}

#[tokio::test]
async fn update_user_error() {
    let user_update_dto = UserUpdateDto {
        role_id: None,
        username: None,
        password: None,
    };

    let mut repo = UserMockRepo::new();
    repo.update_user_result = Some(Err(()));

    let updated = user_service::update_user(&repo, Uuid::new_v4(), user_update_dto)
        .await
        .unwrap_err();

    assert_eq!(UserError::UpdateUserError, updated);
}

//################### soft_delete

#[tokio::test]
async fn soft_delete_success() {
    let mut repo = UserMockRepo::new();
    repo.soft_delete_result = Some(Ok(get_admin_user()));

    let id = Uuid::new_v4();

    let fetched_data = user_service::soft_delete_user(&repo, id).await.unwrap();

    let repo_user_id = (*repo.captured_soft_delete_args.lock().unwrap()).unwrap();

    assert_eq!(repo_user_id, id);
    assert_eq!(fetched_data, ());
}

#[tokio::test]
async fn soft_delete_error() {
    let mut repo = UserMockRepo::new();
    repo.soft_delete_result = Some(Err(()));

    let fetched = user_service::soft_delete_user(&repo, Uuid::new_v4())
        .await
        .err()
        .unwrap();

    assert_eq!(fetched, UserError::SoftDeleteError);
}

#[tokio::test]
async fn soft_delete_does_not_exist() {
    let repo = UserMockRepo::new();

    let fetched = user_service::soft_delete_user(&repo, Uuid::new_v4())
        .await
        .err()
        .unwrap();

    assert_eq!(fetched, UserError::UserNotFound);
}

//################### soft_undelete

#[tokio::test]
async fn soft_undelete_success() {
    let mut repo = UserMockRepo::new();
    repo.soft_undelete_result = Some(Ok(get_admin_user()));

    let id = Uuid::new_v4();

    let fetched_data = user_service::soft_undelete_user(&repo, id).await.unwrap();

    let repo_user_id = (*repo.captured_soft_undelete_args.lock().unwrap()).unwrap();

    assert_eq!(repo_user_id, id);
    assert_eq!(fetched_data, ());
}

#[tokio::test]
async fn soft_undelete_error() {
    let mut repo = UserMockRepo::new();
    repo.soft_undelete_result = Some(Err(()));

    let fetched = user_service::soft_undelete_user(&repo, Uuid::new_v4())
        .await
        .err()
        .unwrap();

    assert_eq!(fetched, UserError::SoftUndeleteError);
}

#[tokio::test]
async fn soft_undelete_does_not_exist() {
    let repo = UserMockRepo::new();

    let fetched = user_service::soft_undelete_user(&repo, Uuid::new_v4())
        .await
        .err()
        .unwrap();

    assert_eq!(fetched, UserError::UserNotFound);
}

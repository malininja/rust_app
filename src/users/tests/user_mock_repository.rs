use std::sync::Mutex;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::Error;
use uuid::Uuid;

use crate::users::{user_model::UserModel, user_repository::UserRepository};

fn get_invalid_user() -> UserModel {
    UserModel {
        id: Uuid::new_v4(),
        role_id: 12345,
        username: "invalid user".to_string(),
        password: "".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: Some(Utc::now()),
    }
}

pub struct UserMockRepo {
    pub get_all_result: Option<Result<Vec<UserModel>, ()>>,
    pub get_by_id_result: Option<Result<UserModel, ()>>,
    pub get_by_username_result: Option<Result<UserModel, ()>>,
    pub create_user_result: Option<Result<UserModel, ()>>,
    pub update_user_result: Option<Result<UserModel, ()>>,
    pub soft_delete_result: Option<Result<UserModel, ()>>,
    pub soft_undelete_result: Option<Result<UserModel, ()>>,
    pub captured_get_by_id_args: Mutex<Option<Uuid>>,
    pub captured_get_by_name_args: Mutex<Option<String>>,
    pub captured_create_args: Mutex<Option<(i32, String, String)>>,
    pub captured_update_args: Mutex<Option<(Uuid, Option<i32>, Option<String>, Option<String>)>>,
    pub captured_soft_delete_args: Mutex<Option<Uuid>>,
    pub captured_soft_undelete_args: Mutex<Option<Uuid>>,
}

impl UserMockRepo {
    pub fn new() -> Self {
        Self {
            get_all_result: None,
            get_by_id_result: None,
            get_by_username_result: None,
            create_user_result: None,
            update_user_result: None,
            soft_delete_result: None,
            soft_undelete_result: None,
            captured_get_by_id_args: None.into(),
            captured_get_by_name_args: None.into(),
            captured_create_args: None.into(),
            captured_update_args: None.into(),
            captured_soft_delete_args: None.into(),
            captured_soft_undelete_args: None.into(),
        }
    }
}

#[async_trait]
impl UserRepository for &UserMockRepo {
    async fn get_all_users(&self) -> Result<Vec<UserModel>, Error> {
        match &self.get_all_result {
            Some(Ok(res)) => Ok(res.clone()),
            Some(Err(())) => Err(Error::RowNotFound),
            None => Ok(vec![get_invalid_user()]),
        }
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserModel>, Error> {
        *self.captured_get_by_id_args.lock().unwrap() = Some(id);

        match &self.get_by_id_result {
            Some(Ok(res)) => Ok(Some(res.clone())),
            Some(Err(())) => Err(Error::RowNotFound),
            None => Ok(None),
        }
    }

    async fn get_user_by_username(&self, username: String) -> Result<Option<UserModel>, Error> {
        *self.captured_get_by_name_args.lock().unwrap() = Some(username);

        match &self.get_by_username_result {
            Some(Ok(res)) => Ok(Some(res.clone())),
            Some(Err(())) => Err(Error::RowNotFound),
            None => Ok(None),
        }
    }

    async fn create_user(
        &self,
        role_id: i32,
        username: String,
        hashed_password: String,
    ) -> Result<UserModel, Error> {
        *self.captured_create_args.lock().unwrap() = Some((role_id, username, hashed_password));

        match &self.create_user_result {
            Some(Ok(res)) => Ok(res.clone()),
            Some(Err(())) => Err(Error::RowNotFound),
            None => Ok(get_invalid_user()),
        }
    }

    async fn update_user(
        &self,
        id: Uuid,
        role_id: Option<i32>,
        username: Option<String>,
        hashed_password: Option<String>,
    ) -> Result<Option<UserModel>, Error> {
        *self.captured_update_args.lock().unwrap() = Some((id, role_id, username, hashed_password));

        match &self.update_user_result {
            Some(Ok(res)) => Ok(Some(res.clone())),
            Some(Err(())) => Err(Error::RowNotFound),
            None => Ok(None),
        }
    }

    async fn soft_delete_user(&self, id: Uuid) -> Result<Option<UserModel>, Error> {
        *self.captured_soft_delete_args.lock().unwrap() = Some(id);

        match &self.soft_delete_result {
            Some(Ok(res)) => Ok(Some(res.clone())),
            Some(Err(())) => Err(Error::RowNotFound),
            None => Ok(None),
        }
    }

    async fn soft_undelete_user(&self, id: Uuid) -> Result<Option<UserModel>, Error> {
        *self.captured_soft_undelete_args.lock().unwrap() = Some(id);

        match &self.soft_undelete_result {
            Some(Ok(res)) => Ok(Some(res.clone())),
            Some(Err(())) => Err(Error::RowNotFound),
            None => Ok(None),
        }
    }
}

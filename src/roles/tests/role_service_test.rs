use async_trait::async_trait;
use chrono::Utc;
use sqlx::Error;
use uuid::Uuid;

use crate::roles::{
    role_error::RoleError, role_model::RoleModel, role_repository::RoleRepository,
    role_service::get_all_roles,
};

#[tokio::test]
async fn get_all_roles_success() {
    struct MockRepo {
        mocks: Vec<RoleModel>,
    }

    impl MockRepo {
        pub fn new(mocks: Vec<RoleModel>) -> Self {
            Self { mocks }
        }
    }

    #[async_trait]
    impl RoleRepository for MockRepo {
        async fn get_all_roles(&self) -> Result<Vec<RoleModel>, Error> {
            Ok(self.mocks.clone())
        }
    }

    let role_model_mocks = vec![
        RoleModel {
            id: Uuid::new_v4(),
            code: String::from("A"),
            name: String::from("mock one"),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        RoleModel {
            id: Uuid::new_v4(),
            code: String::from("B"),
            name: String::from("mock two"),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ];

    let repo = MockRepo::new(role_model_mocks.clone());

    let fetched_data = get_all_roles(repo).await.unwrap();

    assert_eq!(fetched_data, role_model_mocks);
}

#[tokio::test]
async fn get_all_roles_error() {
    struct MockRepo {}

    #[async_trait]
    impl RoleRepository for MockRepo {
        async fn get_all_roles(&self) -> Result<Vec<RoleModel>, Error> {
            Err(Error::RowNotFound)
        }
    }

    let repo = MockRepo {};

    let fetched_data = get_all_roles(repo).await.err().unwrap();

    assert_eq!(fetched_data, RoleError::GetRolesError);
}

use crate::roles::{role_error::RoleError, role_model::RoleModel, role_repository::RoleRepository};

pub async fn get_all_roles<R: RoleRepository>(repository: R) -> Result<Vec<RoleModel>, RoleError> {
    repository.get_all_roles().await.map_err(|e| {
        println!("role_service: Get all roles error: {}", e);
        RoleError::GetRolesError
    })
}

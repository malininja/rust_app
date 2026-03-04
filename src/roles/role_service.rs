use crate::roles::{role_error::RoleError, role_model::RoleModel, role_repository::RoleRepository};

pub async fn get_all_roles<R: RoleRepository>(repository: R) -> Result<Vec<RoleModel>, RoleError> {
    match repository.get_all_roles().await {
        Ok(res) => Ok(res),
        Err(e) => {
            println!("Get all roles error: {}", e);
            Err(RoleError::GetRolesError)
        }
    }
}

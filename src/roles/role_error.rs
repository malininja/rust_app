use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RoleError {
    #[error("Error fetching roles from database")]
    GetRolesError,
}

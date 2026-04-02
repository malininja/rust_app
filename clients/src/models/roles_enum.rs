#[derive(Debug)]
pub enum Role {
    Admin,
    Sales,
    Warehouse,
}

impl From<&Role> for i32 {
    fn from(value: &Role) -> Self {
        match value {
            Role::Admin => 1,
            Role::Sales => 2,
            Role::Warehouse => 3,
        }
    }
}

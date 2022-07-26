use std::fmt::Display;

use serde::Deserialize;

use super::id::{hex_id, UserId};
use super::role::{Role, RoleOrRoles};

#[derive(Debug)]
pub struct User {
    id: UserId,
    username: String,
    roles: Vec<Role>,
}

#[derive(Deserialize)]
pub struct UserRaw {
    pub id: UserId,
    pub username: String,
    pub roles: RoleOrRoles,
}

impl Into<User> for UserRaw {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            roles: self.roles.into(),
        }
    }
}

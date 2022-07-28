

use serde::Deserialize;

use super::id::{UserId};
use super::role::{Role, RoleOrRoles};

#[derive(Debug)]
pub struct User {
    id: UserId,
    username: String,
    roles: Vec<Role>,
}

#[derive(Deserialize)]
pub struct PartialSettings {
	#[serde(rename = "ProfilePic")]
	pub avatar_id: String
}

#[derive(Deserialize)]
pub struct UserRaw {
	#[serde(rename = "_id")]
    pub id: UserId,
	#[serde(rename = "User")]
    pub username: String,
	#[serde(rename = "Role", default)]
    pub roles: RoleOrRoles,
	#[serde(rename = "Settings")]
	pub settings: PartialSettings
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

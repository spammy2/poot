

use serde::Deserialize;

use crate::Auth;
use crate::context::Context;

use super::id::{UserId, Id, AsId};
use super::role::{Role, RoleOrRoles};

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub roles: Vec<Role>,
	pub settings: PartialUserSettings,
}

#[derive(Clone, Deserialize,Debug)]
pub struct PartialUserSettings {
	#[serde(rename = "ProfilePic")]
	pub avatar_id: String
}

impl Default for PartialUserSettings {
	fn default() -> Self {
		PartialUserSettings {
			// https://app.photop.live/?from=launchpage&chat=62e2378c277a80d8ea0d3f4f
			avatar_id: "DefaultProfilePic".to_owned()
		}
	}
}

impl From<UserSettings> for PartialUserSettings {
	fn from(settings: UserSettings) -> Self {
		PartialUserSettings {
			avatar_id: settings.avatar_id
		}
	}
}

impl User {
	pub fn is_self(&self, ctx: &Context) -> bool {
		match &ctx.auth {
			Auth::Token { user_id, .. } => UserId::from(user_id.as_ref()) == self.id,
			_ => false,
		}
	}
}

#[derive(Clone, Deserialize)]
pub struct UserSettings {
	#[serde(rename = "ProfilePic")]
	pub avatar_id: String,
}

#[derive(Deserialize)]
pub struct UserRaw {
	#[serde(rename = "_id")]
    pub id: UserId,
	#[serde(rename = "User")]
    pub username: String,
	#[serde(rename = "Role", default)]
    pub roles: RoleOrRoles,
	#[serde(rename = "Settings", default)]
	pub settings: PartialUserSettings
}

impl Into<User> for UserRaw {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            roles: self.roles.into(),
			settings: self.settings.into()
        }
    }
}

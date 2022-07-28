use chrono::{DateTime, Utc};

use super::{id::ChatId, user::User};

pub struct Chat {
	pub id: ChatId,
	pub content: String,
	pub author: User,
	pub created_at: DateTime<Utc>,
}

impl PartialEq for Chat {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
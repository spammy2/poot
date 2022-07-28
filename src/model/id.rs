use std::{hash::{Hash, Hasher}, fmt::Display};

use serde::{Deserialize, Serialize, de::Visitor};

use super::{group::Group, post::Post, user::User, chat::Chat};

pub fn date_from_u64(id: u128) -> chrono::DateTime<chrono::Utc> {
    let time = id >> 8;
    return chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(time as i64, 0),
        chrono::Utc,
    );
}

pub struct Id<T> {
    phantom: std::marker::PhantomData<T>,
    id: u128,
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:12x}", self.id))
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
		struct HexVisitor<T>(std::marker::PhantomData<T>);
		impl<'de,T> Visitor<'de> for HexVisitor<T> {
			type Value = Id<T>;
			fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Id<T>, E> {
				Ok(Id::from(s))
			}
			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a hex string")
			}
		}

		deserializer.deserialize_str::<HexVisitor<T>>(HexVisitor(std::marker::PhantomData))
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id {
            phantom: std::marker::PhantomData,
            id: self.id,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Id<T> {
    pub fn get_date(&self) -> chrono::DateTime<chrono::Utc> {
        date_from_u64(self.id)
    }
    pub fn to_string(&self) -> String {
        format!("{:12x}", self.id)
    }
}

impl<T> std::fmt::Debug for Id<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl<T> Display for Id<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl<T> From<&str> for Id<T> {
    fn from(id: &str) -> Self {
        Id {
            phantom: std::marker::PhantomData,
            id: u128::from_str_radix(id, 16).unwrap(),
        }
    }
}

pub type UserId = Id<User>;
pub type GroupId = Id<Group>;
pub type PostId = Id<Post>;
pub type ChatId = Id<Chat>;
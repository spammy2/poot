use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use super::{group::Group, post::Post, user::User};

pub fn date_from_u64(id: u128) -> chrono::DateTime<chrono::Utc> {
	let time = id >> 8;
	return chrono::DateTime::<chrono::Utc>::from_utc(
		chrono::NaiveDateTime::from_timestamp(time as i64, 0),
		chrono::Utc,
	);
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Id<T> {
	phantom: std::marker::PhantomData<T>,
	#[serde(with = "hex_id")]
	id: u128,
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

impl<T> Id<T> {
	pub fn get_date(&self) -> chrono::DateTime<chrono::Utc> {
		date_from_u64(self.id)
	}
	pub fn to_string(&self) -> String {
		format!("{:12x}", self.id)
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

pub mod hex_id {
	use std::num::ParseIntError;

	use serde::de::Visitor;
	use serde::Deserializer;
	use serde::Serializer;

	pub fn serialize<S>(val: &u128, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		s.serialize_str(&format!("{:12x}", val))
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct HexVisitor;
		impl<'de> Visitor<'de> for HexVisitor {
			type Value = u128;
			fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<u128, E> {
				println!("{}", s);
				u128::from_str_radix(s, 16).map_err(|_| serde::de::Error::custom("Failed to parse"))
			}

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a hex string")
			}
		}

		deserializer.deserialize_str(HexVisitor)
	}
}

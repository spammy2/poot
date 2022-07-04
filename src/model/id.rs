pub fn date_from_u64(id: u128) -> chrono::DateTime<chrono::Utc> {
	let time = id >> 8;
	return chrono::DateTime::<chrono::Utc>::from_utc(
		chrono::NaiveDateTime::from_timestamp(time as i64, 0),
		chrono::Utc
	);
}
pub trait Id {
	fn get_date(&self)->chrono::DateTime<chrono::Utc>;
}

pub mod hex_id {
    use std::num::ParseIntError;

    use serde::Deserializer;
    use serde::Serializer;
    use serde::de::Visitor;

    pub fn serialize<S>(val: &u128, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        s.serialize_str(&format!("{:12X}", val))
    }

    pub fn deserialize<'de, D>(deserializer: D)
        -> Result<u128, D::Error>
        where D: Deserializer<'de> {
        
        struct HexVisitor;
        impl<'de> Visitor<'de> for HexVisitor {
            type Value = u128;
            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<u128, E> {
                println!("{}", s);
                u128::from_str_radix(s, 16).map_err(|_|serde::de::Error::custom("Failed to parse"))
            }

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a hex string")
            }
        }

        deserializer.deserialize_str(HexVisitor)
    }
}
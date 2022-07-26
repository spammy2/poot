use serde::Deserialize;

#[derive(Debug)]
pub enum Status {
    Offline,
    Online,
    InGroup,
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = u32::deserialize(deserializer)?;
        match s {
            0 => Ok(Status::Offline),
            1 => Ok(Status::Online),
            2 => Ok(Status::InGroup),
            _ => Err(serde::de::Error::custom("invalid status")),
        }
    }
}

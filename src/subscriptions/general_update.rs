use std::sync::Arc;

use crate::{
    model::id::{GroupId, PostId, UserId},
    Context,
};
use serde::{de::Error, Deserialize, Serialize};
use serde_json::Value;
use simplesocket::context::Subscriber;

#[derive(Serialize)]
pub enum GeneralUpdateLocation {
    Home,
}

#[derive(Serialize)]
#[serde(tag = "Task")]
pub struct GeneralUpdate {
    #[serde(rename = "Location")]
    pub location: GeneralUpdateLocation,
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<GroupId>,
}

#[derive(Deserialize, Debug)]
pub struct NewPostAdded {
    #[serde(rename = "Timestamp")]
    pub timestamp: u64,
    #[serde(rename = "UserID")]
    pub user_id: UserId,
    #[serde(rename = "_id")]
    pub post_id: PostId,
}

#[derive(Debug)]
pub enum GeneralUpdateEvent {
    NewPostAdded(NewPostAdded),
}

impl<'de> Deserialize<'de> for GeneralUpdateEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;
        let v = v.as_object().ok_or(Error::custom("shit"))?;

        match v.get("Type").unwrap().as_str().unwrap() {
            "NewPostAdded" => Ok(GeneralUpdateEvent::NewPostAdded(
                serde_json::from_value(v.get("NewPostData").unwrap().to_owned()).unwrap(),
            )),
            _ => Err(Error::custom("Not a valid type")),
        }
    }
}

pub(crate) struct GeneralUpdateSubscriber {
    pub ctx: Arc<Context>,
}

impl Subscriber for GeneralUpdateSubscriber {
    fn callback(&self, event: Value) {
        let event: GeneralUpdateEvent = serde_json::from_value(event).unwrap();
        match event {
            GeneralUpdateEvent::NewPostAdded(post) => {
                let ctx = self.ctx.clone();
                tokio::spawn(async move {
                    let post = ctx.get_post(post.post_id).await.unwrap();
                    ctx.events.on_post(ctx.clone(), post);
                });
            }
            _ => {
                println!("Unknnown event: {:?}", event);
            }
        }
    }
}

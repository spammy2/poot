use std::{sync::{Weak, Mutex, Arc}, cell::RefCell};

use crate::{
    model::id::{GroupId, PostId, UserId},
    Context, Auth,
};
use serde::{de::Error, Deserialize, Serialize};
use serde_json::{Value, json};
use simplesocket::context::Subscriber;

#[derive(Serialize)]
pub enum GeneralUpdateLocation {
	#[serde(rename = "home")]
    Home,
}

#[derive(Serialize)]
#[serde(tag = "task", rename="general")]
pub struct GeneralUpdate {
    #[serde(rename = "location")]
    pub location: GeneralUpdateLocation,
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<GroupId>,

	#[serde(skip_serializing_if = "Auth::is_none")]
	#[serde(flatten)]
	pub auth: Auth,
}

#[test]
fn test_ser(){
	let general_update = GeneralUpdate {
		location: GeneralUpdateLocation::Home,
		groups: vec![],
		auth: Auth::Token { user_id: "6066b99198895e660082965b".to_owned(), token: "joaojifaeijofiojawfoijaw".to_owned() },
	};
	let json = serde_json::to_string(&general_update).unwrap();
	println!("{}", json);
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

#[test]
fn test_new_post_deser(){
	let json = json!({
		"Timestamp": 1658965141339 as u64,
		"UserID": "622fb3400c9eb9061377ddda",
		"_id": "62e1cc95285e4588ae587085"
	});
	let new_post_added: NewPostAdded = serde_json::from_value(json).unwrap();
	println!("{:?}", new_post_added);
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

        match v.get("type").unwrap().as_str().unwrap() {
            "newpost" => Ok(GeneralUpdateEvent::NewPostAdded(
                serde_json::from_value(v.get("post").unwrap().to_owned()).unwrap(),
            )),
            _ => Err(Error::custom("Not a valid type")),
        }
    }
}

pub(crate) struct GeneralUpdateSubscriber  {
    pub ctx: Arc<Mutex<Option<Context>>>,
}

impl Subscriber for GeneralUpdateSubscriber {
    fn callback(&self, event: Value) {
        let event: GeneralUpdateEvent = serde_json::from_value(event).unwrap();
        match event {
            GeneralUpdateEvent::NewPostAdded(post) => {
				let lock = self.ctx.lock().unwrap();
                let ctx = lock.as_ref().expect("Value is not exist").clone();
				drop(lock);
                tokio::spawn(async move {
                    let post = ctx.get_post(post.post_id).await.unwrap();
                    ctx.events.on_post(ctx.clone(), post).await;
                });
            }
            _ => {
                println!("Unknnown event: {:?}", event);
            }
        }
    }
}

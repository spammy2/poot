use serde::Serialize;
use serde_json::json;

use crate::model::{id::{GroupId, PostId}, post::Post};

use super::BASE_API_URL;

#[derive(Serialize)]
pub struct CreatePostBody {
    #[serde(rename = "text")]
    pub content: String,

    #[serde(skip_serializing)]
    pub group_id: Option<GroupId>,

    #[serde(skip_serializing)]
    pub images: Vec<reqwest::multipart::Part>,
}

impl From<String> for CreatePostBody {
    fn from(content: String) -> CreatePostBody {
        CreatePostBody {
            content,
            group_id: None,
            images: Vec::new(),
        }
    }
}

impl super::Context {
    pub async fn create_post(&self, body: CreatePostBody) -> Result<PostId, reqwest::Error> {
        let CreatePostBody {
            content,
            group_id,
            mut images,
        } = body;

        let form = images.drain(..).enumerate().fold(
            reqwest::multipart::Form::new().text("data", json!({ "text": content }).to_string()),
            |form, (i, image)| form.part(format!("image_{}", i), image),
        );

        let mut post_url = BASE_API_URL.join("posts/new").unwrap();
        if let Some(group_id) = group_id {
            post_url
                .query_pairs_mut()
                .append_pair("groupid", &group_id.to_string());
            println!("{}", group_id.to_string());
        }

        let post_id = self
            .client
            .post(post_url)
            .header("auth", self.auth.to_string())
            .multipart(form)
            .send()
            .await?.text().await?;
		Ok(PostId::from(&post_id[..]))
		// Ok(self.get_post(post_id).await?)
    }
}

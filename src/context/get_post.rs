use serde::Deserialize;

use crate::{model::{user::UserRaw, id::PostId, post::{Post, PostRaw}}, context::BASE_API_URL};


impl super::Context {
    pub async fn get_post(&self, id: PostId) -> Result<Post, reqwest::Error> {
        #[derive(Deserialize)]
        struct GetResponse {
            users: Vec<UserRaw>,
            posts: Vec<PostRaw>,
        }

        let mut url = BASE_API_URL.join("posts").unwrap();
        url.query_pairs_mut().append_pair(
            "postid",
            &id.to_string(),
        );

        let val = self
            .client
            .get(url)
            .send()
            .await?
            .json::<GetResponse>()
            .await?;

        let GetResponse {
            mut posts,
            mut users,
        } = val;
        let post = posts.remove(0);
        let user = users.remove(0);
        let post = Post {
            id: post.id,
            author: user.into(),
            content: post.content,
        };
        return Ok(post);
    }
}

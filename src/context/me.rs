use std::collections::HashMap;

use serde::Deserialize;

use crate::{model::{client_user::{ClientUserRaw, ClientUser}, id::GroupId, group::RawGroup}, context::BASE_API_URL};

impl super::Context {
	pub async fn me(&self) -> Result<ClientUser, reqwest::Error> {
        #[derive(Deserialize)]
        struct GetResponse {
            groups: HashMap<GroupId, RawGroup>,
            user: ClientUserRaw,
        }

        let mut url = BASE_API_URL.join("me").unwrap();
        url.query_pairs_mut()
            .append_pair("ss", &self.simplesocket.get_secure_id()[..]);
        let val = self
            .client
            .get(url)
            .header("auth", self.auth.to_string())
            .send()
            .await?
            .json::<GetResponse>()
            .await?;

        let GetResponse { user, groups } = val;

        Ok(ClientUser {
            id: user.id,
            created_at: user.creation_time,
            infractions: user.infractions,
            last_important_update: user.last_important_update,
            last_login: user.last_login,
            logins: user.logins,
            description: user.description,
            followers: user.followers,
            following: user.following,
            groups: groups.into_iter().map(|(k, v)| v.into_group(k)).collect(),
            username: user.username,
            status: user.status,
            email: user.email,
        })
    }
}
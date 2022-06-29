use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub groups: Vec<Group>,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Invite")]
    pub invite: String,
    #[serde(rename = "LastChecked")]
    pub last_checked: i64,
    #[serde(rename = "LastContent")]
    pub last_content: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: i64,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "BlockedUsers")]
    pub blocked_users: Vec<Value>,
    #[serde(rename = "CreationTime")]
    pub creation_time: i64,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "LastImportantUpdate")]
    pub last_important_update: i64,
    #[serde(rename = "LastLogin")]
    pub last_login: i64,
    #[serde(rename = "LastUserDelete")]
    pub last_user_delete: i64,
    #[serde(rename = "Logins")]
    pub logins: i64,
    #[serde(rename = "Premium")]
    pub premium: Premium,
    #[serde(rename = "ProfileData")]
    pub profile_data: ProfileData,
    #[serde(rename = "Role")]
    pub role: Vec<Value>,
    #[serde(rename = "Settings")]
    pub settings: Settings,
    #[serde(rename = "Status")]
    pub status: i64,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Premium {
    #[serde(rename = "Bought")]
    pub bought: String,
    #[serde(rename = "Expires")]
    pub expires: String,
    #[serde(rename = "Renewing")]
    pub renewing: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileData {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Followers")]
    pub followers: i64,
    #[serde(rename = "Following")]
    pub following: i64,
    #[serde(rename = "PinnedPost")]
    pub pinned_post: String,
    #[serde(rename = "Socials")]
    pub socials: Socials,
    #[serde(rename = "Visibility")]
    pub visibility: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Socials {
    #[serde(rename = "discord_[SOCIAL_USER_ID]")]
    pub discord_social_user_id: String,
    #[serde(rename = "github_[SOCIAL_USER_ID]")]
    pub github_social_user_id: String,
    #[serde(rename = "instagram_[SOCIAL_USER_ID]")]
    pub instagram_social_user_id: String,
    #[serde(rename = "pinterest_[SOCIAL_USER_ID]")]
    pub pinterest_social_user_id: String,
    #[serde(rename = "reddit_[SOCIAL_USER_ID]")]
    pub reddit_social_user_id: String,
    #[serde(rename = "twitch_[SOCIAL_USER_ID]")]
    pub twitch_social_user_id: String,
    #[serde(rename = "twitter_[SOCIAL_USER_ID]")]
    pub twitter_social_user_id: String,
    #[serde(rename = "youtube_[SOCIAL_USER_ID]")]
    pub youtube_social_user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "Display")]
    pub display: Display,
    #[serde(rename = "ProfileBanner")]
    pub profile_banner: String,
    #[serde(rename = "ProfilePic")]
    pub profile_pic: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Display {
    #[serde(rename = "Embed GIFs")]
    pub embed_gifs: bool,
    #[serde(rename = "Embed Twitch Live Chat:")]
    pub embed_twitch_live_chat: bool,
    #[serde(rename = "Embed Twitch Streams:")]
    pub embed_twitch_streams: bool,
    #[serde(rename = "Embed YouTube Videos:")]
    pub embed_you_tube_videos: bool,
    #[serde(rename = "Theme:")]
    pub theme: String,
}
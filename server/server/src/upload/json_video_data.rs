use chrono::{DateTime, Utc};
use mime::Mime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonVideoData {
    #[serde(with = "super::mime_serde")]
    pub content_type: Mime,
    pub upload_at: DateTime<Utc>,
    pub video_platform: VideoPlatform,

    pub video_title: Box<str>,
    pub description: Box<str>,
    pub hashtags: Vec<Box<str>>,
    pub additional: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum VideoPlatform {
    TikTok,
    YoutubeShorts,
    InstagramReels,
}

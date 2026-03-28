use axum::{Json, extract::Request};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    json_multipart::JsonMultiPart,
    upload::{JsonVideoData, UploadError, VideoPlatform},
};

use super::{
    file_system::{ensure_videos_directory, videos_folder},
    save_files::save_files,
    unique_video_uuid::unique_video_uuid,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UploadRequest {
    pub upload_at: DateTime<Utc>,
    pub video_platform: VideoPlatform,

    pub video_title: Box<str>,
    pub description: Box<str>,
    pub hashtags: Vec<Box<str>>,
    pub additional: serde_json::Value,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub uuid: Uuid,
}

pub async fn upload(request: Request) -> Result<Json<UploadResponse>, UploadError> {
    let json_with_file: JsonMultiPart<UploadRequest> = JsonMultiPart::try_new(request).await?;
    let videos_folder = videos_folder()?;
    ensure_videos_directory(videos_folder.as_path()).await?;
    let uuid = unique_video_uuid(videos_folder.as_path()).await?;

    save_files(
        uuid,
        videos_folder.as_path(),
        JsonVideoData {
            content_type: json_with_file.video_content_type,
            upload_at: json_with_file.json.upload_at,
            video_platform: json_with_file.json.video_platform,
            video_title: json_with_file.json.video_title,
            description: json_with_file.json.description,
            hashtags: json_with_file.json.hashtags,
            additional: json_with_file.json.additional,
        },
        &json_with_file.video,
    )
    .await?;

    Ok(Json(UploadResponse { uuid }))
}

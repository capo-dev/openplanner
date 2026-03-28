use std::path::Path;

use uuid::Uuid;

use crate::upload::UploadError;

use super::JsonVideoData;

pub async fn save_files(
    uuid: Uuid,
    videos_folder: &Path,
    json_video_data: JsonVideoData,
    video: &[u8],
) -> Result<(), UploadError> {
    let uuid_string = uuid.to_string();

    let (a, b) = tokio::join!(
        save_json(uuid_string.as_str(), videos_folder, json_video_data),
        save_video(uuid_string.as_str(), videos_folder, video)
    );
    a?;
    b
}

async fn save_json(
    uuid: &str,
    videos_folder: &Path,
    json_video_data: JsonVideoData,
) -> Result<(), UploadError> {
    let json_file = {
        let mut json_file = videos_folder.join(uuid);
        json_file.set_extension("json");
        json_file
    };

    let serialized = serde_json::to_string(&json_video_data)
        .map_err(|err| UploadError::Json(err.to_string().into_boxed_str()))?;

    match tokio::fs::write(json_file.as_path(), serialized).await {
        Ok(()) => Ok(()),
        Err(err) => Err(UploadError::Io {
            path: json_file,
            err: err.to_string().into_boxed_str(),
        }),
    }
}

async fn save_video(uuid: &str, videos_folder: &Path, video: &[u8]) -> Result<(), UploadError> {
    let video_file = videos_folder.join(uuid);

    match tokio::fs::write(video_file.as_path(), video).await {
        Ok(()) => Ok(()),
        Err(err) => Err(UploadError::Io {
            path: video_file,
            err: err.to_string().into_boxed_str(),
        }),
    }
}

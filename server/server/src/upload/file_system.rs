use std::path::{Path, PathBuf};

use server_macros::proj;

use crate::upload::UploadError;

const VIDEOS_FOLDER: &str = proj!("VIDEOS_FOLDER");

pub fn videos_folder() -> Result<PathBuf, UploadError> {
    let mut path = std::env::current_dir()
        .map_err(|err| UploadError::Cwd(err.to_string().into_boxed_str()))?;
    path.push(VIDEOS_FOLDER);
    Ok(path)
}

pub async fn ensure_videos_directory(path: &Path) -> Result<(), UploadError> {
    let err = match tokio::fs::try_exists(path).await {
        Ok(true) => return Ok(()),
        Ok(false) => match tokio::fs::create_dir(path).await {
            Ok(()) => return Ok(()),
            Err(err) => err,
        },
        Err(err) => err,
    };

    Err(UploadError::Io {
        path: path.to_path_buf(),
        err: err.to_string().into_boxed_str(),
    })
}

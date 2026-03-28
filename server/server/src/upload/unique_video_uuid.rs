use std::{collections::HashSet, ffi::OsStr, path::Path, str::FromStr};

use uuid::Uuid;

use crate::upload::UploadError;

pub async fn unique_video_uuid(path: &Path) -> Result<Uuid, UploadError> {
    let mut entries = match tokio::fs::read_dir(path).await {
        Ok(ok) => ok,
        Err(err) => {
            return Err(UploadError::Io {
                path: path.to_path_buf(),
                err: err.to_string().into_boxed_str(),
            });
        }
    };

    let mut current_uuids = HashSet::new();
    while let Some(entry_result) = entries.next_entry().await.transpose() {
        let entry = match entry_result {
            Ok(ok) => ok,
            Err(err) => {
                return Err(UploadError::Io {
                    path: path.to_path_buf(),
                    err: err.to_string().into_boxed_str(),
                });
            }
        };
        let filepath = entry.path();
        let fileprefix = match filepath.file_prefix().and_then(OsStr::to_str) {
            Some(some) => some,
            None => continue,
        };
        let uuid = match Uuid::from_str(fileprefix) {
            Ok(ok) => ok,
            Err(_) => continue,
        };
        current_uuids.insert(uuid);
    }

    loop {
        let new_uuid = Uuid::new_v4();

        if !current_uuids.contains(&new_uuid) {
            return Ok(new_uuid);
        }
    }
}

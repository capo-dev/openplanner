use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::json_multipart::{JsonMultiPart, JsonMultiPartError};

#[derive(Debug, Deserialize)]
pub struct UploadRequest {}

#[derive(Serialize)]
pub struct UploadResponse {
    pub uuid: Uuid,
}

pub async fn upload(request: Request) -> Result<Json<UploadResponse>, UploadError> {
    let json_with_file: JsonMultiPart<UploadRequest> = JsonMultiPart::try_new(request).await?;

    println!("{:#?}", json_with_file.json);
    println!("{:#?}", json_with_file.file);

    let uuid = Uuid::new_v4();

    Ok(Json(UploadResponse { uuid }))
}

#[derive(Debug, Error, Serialize)]
pub enum UploadError {
    #[error(transparent)]
    JsonMultiPart(#[from] JsonMultiPartError),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

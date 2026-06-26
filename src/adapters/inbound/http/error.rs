use crate::application::use_cases::get_next_question::GetNextQuestionError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!(error = %self);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl From<GetNextQuestionError> for AppError {
    fn from(e: GetNextQuestionError) -> Self {
        AppError::Internal(e.to_string())
    }
}

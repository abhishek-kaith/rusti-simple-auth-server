use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
pub type Result<T> = std::result::Result<T, AppError>;

pub enum AppError {
    NotFound,
    UnhandledError,
    InternalServerError,
    UserAlreadyExists,
    InvalidCredentials,
    UserNotVerified,
    InvalidVerificationToken,
    Unauthorized,
}

#[derive(Debug, Serialize)]
pub struct CommResp {
    pub status: u16,
    pub message: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::UnhandledError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid Credentials"),
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, "User Already Exists"),
            AppError::UserNotVerified => (StatusCode::UNAUTHORIZED, "User Not Verified"),
            AppError::InvalidVerificationToken => {
                (StatusCode::UNAUTHORIZED, "Invalid Verification Token")
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
        };

        let body = axum::Json(CommResp {
            status: status_code.as_u16(),
            message,
        });

        (status_code, body).into_response()
    }
}

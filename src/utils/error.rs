use thiserror::Error;
use http::StatusCode;

#[derive(Error, Debug, Clone)]
pub enum LoginError {
    #[error("Invalid Password")]
    InvalidPassword,
    #[error("Email is not found")]
    EmailNotFound,
}

impl LoginError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            LoginError::InvalidPassword => StatusCode::UNAUTHORIZED,
            LoginError::EmailNotFound => StatusCode::NOT_FOUND,
        }
    }
}

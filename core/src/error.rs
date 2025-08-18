use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Json(_) => StatusCode::BAD_REQUEST,
            Error::Http(_) => StatusCode::BAD_REQUEST,
        }
    }
}

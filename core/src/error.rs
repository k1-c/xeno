use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal(String),

    #[error("JSON parse error")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error")]
    Http(#[from] http::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Request entity too large")]
    PayloadTooLarge,

    #[error("Request timeout")]
    RequestTimeout,

    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Json(_) => StatusCode::BAD_REQUEST,
            Error::Http(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Error::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            Error::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    pub fn safe_message(&self) -> &'static str {
        match self {
            Error::NotFound => "Not Found",
            Error::BadRequest(_) => "Bad Request",
            Error::Internal(_) => "Internal Server Error",
            Error::Json(_) => "Invalid JSON",
            Error::Http(_) => "HTTP Error",
            Error::Unauthorized => "Unauthorized",
            Error::Forbidden => "Forbidden",
            Error::PayloadTooLarge => "Request Entity Too Large",
            Error::RequestTimeout => "Request Timeout",
            Error::UnprocessableEntity(_) => "Unprocessable Entity",
        }
    }

    pub fn debug_message(&self) -> String {
        self.to_string()
    }

    pub fn bad_request<T: Into<String>>(message: T) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn internal<T: Into<String>>(message: T) -> Self {
        Self::Internal(message.into())
    }

    pub fn not_found() -> Self {
        Self::NotFound
    }

    pub fn unauthorized() -> Self {
        Self::Unauthorized
    }

    pub fn forbidden() -> Self {
        Self::Forbidden
    }

    pub fn payload_too_large() -> Self {
        Self::PayloadTooLarge
    }

    pub fn request_timeout() -> Self {
        Self::RequestTimeout
    }

    pub fn unprocessable_entity<T: Into<String>>(message: T) -> Self {
        Self::UnprocessableEntity(message.into())
    }
}

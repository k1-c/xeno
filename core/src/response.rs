use crate::CoreResponse;
use bytes::Bytes;
use http::StatusCode;
use serde::Serialize;

pub trait IntoResponse {
    fn into_response(self) -> CoreResponse;
}

impl IntoResponse for &str {
    fn into_response(self) -> CoreResponse {
        http::Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Bytes::from(self.to_string()))
            .unwrap()
    }
}

impl IntoResponse for String {
    fn into_response(self) -> CoreResponse {
        self.as_str().into_response()
    }
}

impl IntoResponse for Bytes {
    fn into_response(self) -> CoreResponse {
        http::Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/octet-stream")
            .body(self)
            .unwrap()
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> CoreResponse {
        http::Response::builder()
            .status(self)
            .body(Bytes::new())
            .unwrap()
    }
}

pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> CoreResponse {
        let body = match serde_json::to_vec(&self.0) {
            Ok(bytes) => Bytes::from(bytes),
            Err(_) => {
                return http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Bytes::from("Failed to serialize JSON"))
                    .unwrap()
            }
        };

        http::Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json; charset=utf-8")
            .body(body)
            .unwrap()
    }
}

impl<T: IntoResponse> IntoResponse for (StatusCode, T) {
    fn into_response(self) -> CoreResponse {
        let mut response = self.1.into_response();
        *response.status_mut() = self.0;
        response
    }
}

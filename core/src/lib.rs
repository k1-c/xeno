pub mod app;
pub mod error;
pub mod extract;
pub mod handler;
pub mod middleware;
pub mod response;
pub mod router;
pub mod context;

pub use app::App;
pub use error::Error;
pub use extract::{Json, Path, Query};
pub use handler::Handler;
pub use response::IntoResponse;
pub use context::Ctx;

pub type CoreRequest = http::Request<bytes::Bytes>;
pub type CoreResponse = http::Response<bytes::Bytes>;

pub type Result<T> = std::result::Result<T, Error>;
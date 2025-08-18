pub mod app;
pub mod context;
pub mod error;
pub mod extract;
pub mod handler;
pub mod middleware;
pub mod response;
pub mod router;

pub use app::App;
pub use context::Ctx;
pub use error::Error;
pub use extract::{Json, Path, Query};
pub use handler::Handler;
pub use response::IntoResponse;

pub type CoreRequest = http::Request<bytes::Bytes>;
pub type CoreResponse = http::Response<bytes::Bytes>;

pub type Result<T> = std::result::Result<T, Error>;

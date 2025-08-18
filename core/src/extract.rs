use crate::{CoreRequest, Error};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub struct Path<T>(pub T);

impl<T> Path<T>
where
    T: DeserializeOwned,
{
    pub fn extract(req: &CoreRequest) -> Result<Self, Error> {
        let params = req
            .extensions()
            .get::<HashMap<String, String>>()
            .ok_or_else(|| Error::BadRequest("No path parameters found".to_string()))?;

        let json_value = serde_json::to_value(params)
            .map_err(|e| Error::BadRequest(format!("Failed to convert path params: {}", e)))?;

        let extracted = T::deserialize(json_value)
            .map_err(|e| Error::BadRequest(format!("Failed to deserialize path params: {}", e)))?;

        Ok(Path(extracted))
    }
}

pub struct Query<T>(pub T);

impl<T> Query<T>
where
    T: DeserializeOwned,
{
    pub fn extract(req: &CoreRequest) -> Result<Self, Error> {
        let query_str = req.uri().query().unwrap_or("");

        let params: HashMap<String, String> = url::form_urlencoded::parse(query_str.as_bytes())
            .into_owned()
            .collect();

        let json_value = serde_json::to_value(params)
            .map_err(|e| Error::BadRequest(format!("Failed to convert query params: {}", e)))?;

        let extracted = T::deserialize(json_value)
            .map_err(|e| Error::BadRequest(format!("Failed to deserialize query params: {}", e)))?;

        Ok(Query(extracted))
    }
}

pub struct Json<T>(pub T);

impl<T> Json<T>
where
    T: DeserializeOwned,
{
    pub fn extract(req: &CoreRequest) -> Result<Self, Error> {
        let body = req.body();
        let parsed = serde_json::from_slice(body)?;
        Ok(Json(parsed))
    }
}

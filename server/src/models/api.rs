use axum::{http::StatusCode, Json};
use serde::{Serialize, Serializer, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Response {
    pub status: u16,
    pub message: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionalId {
    #[serde(default = "default_id")]
    pub id: Option<i64>,
}

fn default_id() -> Option<i64> {
    None
}

#[derive(Debug, Clone)]
pub enum Data {
    None,
    One(Value),
    Some(Vec<Value>),
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Data::None => serializer.serialize_none(),
            Data::One(value) => serializer.serialize_some(value),
            Data::Some(values) => serializer.serialize_some(values),
        }
    }
}

impl Response {
    pub fn new(status: StatusCode, message: &str, data: Data) -> Self {
        Self {
            status: status.as_u16(),
            message: message.to_string(),
            data,
        }
    }
    pub fn create(status: StatusCode, message: &str, data: Data) -> Json<Response> {
        Json(Response::new(status, message, data))
    }
}

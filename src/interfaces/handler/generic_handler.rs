use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct AdapterRequest {
    pub query: Option<Value>,
    pub params: Option<Value>,
    pub body: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdapterResponse {
    pub status: StatusCode,
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    InternalServerError = 500,
    NotImplemented = 501,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
}

impl From<u16> for StatusCode {
    fn from(value: u16) -> Self {
        match value {
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            204 => StatusCode::NoContent,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            409 => StatusCode::Conflict,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            _ => StatusCode::InternalServerError,
        }
    }
}

#[async_trait]
pub trait GenericHandler {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse;
}

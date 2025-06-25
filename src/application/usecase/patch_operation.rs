use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PatchOperation {
    op: String,
    path: String,
    value: serde_json::Value,
}

impl PatchOperation {
    pub fn replace<T: serde::Serialize>(path: &str, value: T) -> Self {
        Self {
            op: "replace".to_string(),
            path: path.to_string(),
            value: serde_json::to_value(value).unwrap(),
        }
    }
}

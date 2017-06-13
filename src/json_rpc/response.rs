extern crate serde_json;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub version: String,
    pub result: serde_json::Value
}

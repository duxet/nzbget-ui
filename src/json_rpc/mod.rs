extern crate serde_json;

mod response;

use self::response::Response;
use serde_json::Value;
use std::io::Read;
use hyper::client::Client as HyperClient;
use hyper::header::{ContentType};
#[allow(unused_imports)] use hyper::mime::*;

pub struct Client {
    base_uri: String,
    client: HyperClient
}

#[derive(Serialize)]
struct RequestBody {
    version: String,
    method: String,
    params: Value
}

impl Client {
    pub fn new(base_uri: &str) -> Self {
        Client {
            base_uri: base_uri.to_string(),
            client: HyperClient::new()
        }
    }

    pub fn call_method(&self, method: &str, params: Option<Value>) -> Response {
        let body = RequestBody {
            version: "1.1".to_string(),
            method: method.to_string(),
            params: params.unwrap_or(Value::Array(vec![]))
        };

        let mut response = self.client.post(format!("{}/jsonrpc", self.base_uri).as_str())
            .body(&serde_json::to_string(&body).unwrap())
            .header(ContentType(mime!(Application/Json)))
            .send()
            .unwrap();

        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();

        let response: Response = serde_json::from_str(&body).unwrap();

        response
    }
}

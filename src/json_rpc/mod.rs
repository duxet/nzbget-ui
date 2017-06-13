extern crate serde_json;

mod response;

use self::response::Response;
use std::io::Read;
use hyper::client::Client as HyperClient;
use hyper::header::{ContentType};
#[allow(unused_imports)] use hyper::mime::*;

pub struct Client {
    base_uri: String,
    client: HyperClient
}

impl Client {
    pub fn new(base_uri: &str) -> Self {
        Client {
            base_uri: base_uri.to_string(),
            client: HyperClient::new()
        }
    }

    pub fn call_method(&self, method: &str) -> Response {
        let mut response = self.client.post(format!("{}/jsonrpc", self.base_uri).as_str())
            .body(format!("{{\"method\": \"{}\"}}", method).as_str())
            .header(ContentType(mime!(Application/Json)))
            .send()
            .unwrap();

        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();

        let response: Response = serde_json::from_str(&body).unwrap();

        response
    }
}

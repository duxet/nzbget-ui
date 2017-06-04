extern crate serde_json;

use std::io::Read;
use hyper::client::{Client};
use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

#[derive(Debug, Deserialize)]
pub struct Response {
    pub version: String,
    pub result: serde_json::Value
}

pub fn call_method(method: &str) -> Response {
    let client = Client::new();

    let mut response = client.post("http://localhost:6789/jsonrpc")
        .body(format!("{{\"method\": \"{}\"}}", method).as_str())
        .header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                     vec![(Attr::Charset, Value::Utf8)])))
        .send()
        .unwrap();

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    let response: Response = serde_json::from_str(&body).unwrap();

    response
}

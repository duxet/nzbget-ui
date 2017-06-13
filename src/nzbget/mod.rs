extern crate serde_json;

mod group;
pub use self::group::Group;

mod status;
pub use self::status::Status;

use super::json_rpc;

pub struct Client {
    client: json_rpc::Client
}

impl Client {
    pub fn new(base_uri: &str) -> Self {
        Client { client: json_rpc::Client::new(base_uri) }
    }

    pub fn load_groups(&self) -> Vec<Group> {
        let response = self.client.call_method("listgroups");

        serde_json::from_value(response.result).unwrap()
    }

    pub fn load_status(&self) -> Status {
        let response = self.client.call_method("status");

        serde_json::from_value(response.result).unwrap()
    }

    pub fn pause_download(&self) {
        self.client.call_method("pausedownload");
    }

    pub fn resume_download(&self) {
        self.client.call_method("resumedownload");
    }
}

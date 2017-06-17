extern crate serde_json;

mod group;
pub use self::group::Group;

mod status;
pub use self::status::Status;

use super::json_rpc;

pub struct Client {
    client: json_rpc::Client
}

#[derive(Serialize)]
struct EditQueueParams<'a>(&'a str, &'a str, Vec<u32>);

impl Client {
    pub fn new(base_uri: &str) -> Self {
        Client { client: json_rpc::Client::new(base_uri) }
    }

    pub fn load_groups(&self) -> Vec<Group> {
        let response = self.client.call_method("listgroups", None);

        serde_json::from_value(response.result).unwrap()
    }

    pub fn load_status(&self) -> Status {
        let response = self.client.call_method("status", None);

        serde_json::from_value(response.result).unwrap()
    }

    pub fn pause_download(&self) {
        self.client.call_method("pausedownload", None);
    }

    pub fn resume_download(&self) {
        self.client.call_method("resumedownload", None);
    }

    pub fn delete_groups(&self, group_ids: Vec<u32>) {
        let params = EditQueueParams("GroupDelete", "", group_ids);
        let params = serde_json::to_value(params).unwrap();

        self.client.call_method("editqueue", Some(params));
    }

    pub fn pause_groups(&self, group_ids: Vec<u32>) {
        let params = EditQueueParams("GroupPause", "", group_ids);
        let params = serde_json::to_value(params).unwrap();

        self.client.call_method("editqueue", Some(params));
    }

    pub fn resume_groups(&self, group_ids: Vec<u32>) {
        let params = EditQueueParams("GroupResume", "", group_ids);
        let params = serde_json::to_value(params).unwrap();

        self.client.call_method("editqueue", Some(params));
    }
}

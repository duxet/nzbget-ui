#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Status {
    pub DownloadRate: i32,
    pub DownloadPaused: bool
}

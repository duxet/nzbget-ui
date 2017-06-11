#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Group {
    pub NZBID: u32,
    pub NZBNicename: String,
    pub Status: String,
    pub FileSizeLo: u32,
    pub FileSizeHi: u32,
    pub DownloadedSizeLo: u32,
    pub DownloadedSizeHi: u32,
    pub RemainingSizeLo: u32,
    pub RemainingSizeHi: u32
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Status {
    pub DownloadRate: i32,
    pub DownloadPaused: bool
}

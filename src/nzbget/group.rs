#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Group {
    pub NZBID: u32,
    pub NZBNicename: String,
    pub Status: String,
    FileSizeLo: u32,
    FileSizeHi: u32,
    DownloadedSizeLo: u32,
    DownloadedSizeHi: u32,
    RemainingSizeLo: u32,
    RemainingSizeHi: u32
}

impl Group {
    pub fn file_size(&self) -> u64 {
        format!("{}{}", self.FileSizeHi, self.FileSizeLo).parse::<u64>().unwrap()
    }

    pub fn downloaded_size(&self) -> u64 {
        format!("{}{}", self.DownloadedSizeHi, self.DownloadedSizeLo).parse::<u64>().unwrap()
    }

    pub fn remaining_size(&self) -> u64 {
        format!("{}{}", self.RemainingSizeHi, self.RemainingSizeLo).parse::<u64>().unwrap()
    }

    pub fn progress(&self) -> f32 {
        self.downloaded_size() as f32 / self.file_size() as f32 * 100.0
    }
}

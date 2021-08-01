use super::sheet::{SheetApi};

pub enum Version {
    V4,
}

pub struct Client {
    version: Version,
}

impl Client {

    pub fn new(version: Version) -> Self {
        Self { version }
    }

    pub fn sheet(self) -> SheetApi {
        SheetApi::new()
    }
}

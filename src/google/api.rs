use super::sheet::{SheetApi};

pub enum Version {
    V4,
}

pub struct GoogleApi {
    version: Version,
}

impl GoogleApi {

    pub fn new(version: Version) -> Self {
        Self { version }
    }

    pub fn sheet(self) -> SheetApi {
        SheetApi::new()
    }
}

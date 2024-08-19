use std::io::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    Idle,
    Pending,
    Downloading,
    Montage,
    Uploading,
    Success,
    Skipping,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Process {
    status: Status,
    key: String,

}

impl Process {
    pub async fn new(key: &String) -> Result<Self, Error> {
        Ok(
            Process {
                status: Status::Pending,
                key: key.to_string(),
            }
        )
    }

    pub fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
    }
}
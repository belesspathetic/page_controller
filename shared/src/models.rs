// pub struct Keys(Vec<String>);

use crate::api::get_me_api::get_me_api;

use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub struct Page {
    pub name: String,
    pub id: String,
    pub followers_count: u32,
    pub key: Option<String>
}

impl Page {
    pub async fn new(key: String) -> Result<Self, reqwest::Error> {
        let resp = get_me_api(&key).await?;

        Ok(Page {
            name: resp.name,
            id: resp.id,
            followers_count: resp.followers_count,
            key: Some(key),
        })
    }
}
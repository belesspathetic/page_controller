// pub struct Keys(Vec<String>);

use crate::api::get_me_api::get_me_api;

use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone)]
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

    pub fn default() -> Self {
        Page {
            name: "".to_string(),
            id: "".to_string(),
            followers_count: 0,
            key: Some("None".to_string()),
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ManualData {
    pub key: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub tags: Option<String>,
    pub video_or_reels: Option<String>,
    pub montage_or_no_montage: Option<String>,
}

impl ManualData {
    pub fn new() -> Self {
        ManualData::default()
    }

    // Method to add a key
    pub fn add_key(mut self, key: &str) -> Self {
        self.key = Some(key.to_string());
        self
    }

    // Method to add a title
    pub fn add_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    // Method to add tags
    pub fn add_tags(mut self, tags: &str) -> Self {
        self.tags = Some(tags.to_string());
        self
    }

    // Method to add video_or_reels
    pub fn video_or_reels(mut self, video_or_reels: &str) -> Self {
        self.video_or_reels = Some(video_or_reels.to_string());
        self
    }

    // Method to add montage_or_no_montage
    pub fn montage_or_no_montage(mut self, montage_or_no_montage: &str) -> Self {
        self.montage_or_no_montage = Some(montage_or_no_montage.to_string());
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
}

impl Default for ManualData {
    fn default() -> Self {
        ManualData {
            key: None,
            url: None,
            title: None,
            tags: None,
            video_or_reels: None,
            montage_or_no_montage: None,
        }
    }
}
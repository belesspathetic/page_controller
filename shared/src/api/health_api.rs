use crate::consts::BACK;
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use reqwest::Error;
use reqwest::{Client, StatusCode};

pub async fn fb_health_check_api() -> Result<StatusCode, Error> {
    let cl = Client::new();

    let endpoint = format!("http://{}/fb_health_check", BACK);

    let resp = cl
        .get(endpoint)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .send()
        .await?;

    Ok(resp.status())
}

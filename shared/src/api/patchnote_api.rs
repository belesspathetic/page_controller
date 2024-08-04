use crate::consts::BACK;
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use reqwest::Client;
use reqwest::{Error, Response};

pub async fn patchnote_api() -> Result<Response, Error> {
    let cl = Client::new();

    let endpoint = format!("http://{}/patchnote", BACK);

    let resp = cl
        .get(endpoint)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .send()
        .await?;

    Ok(resp)
}

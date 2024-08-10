use crate::consts::BACK;
use crate::models::Page;
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use reqwest::Error;
use reqwest::Client;

pub async fn get_me_api(key: &String) -> Result<Page, Error> {
    let cl = Client::new();

    let endpoint = format!("http://{}/get_me", BACK);

    let resp = cl
        .get(endpoint)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .query(&[("key", key)])
        .send()
        .await?;

    let page = resp.json::<Page>().await?;

    Ok(page)
}
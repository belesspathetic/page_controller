use consts::{FB_URL, GRAPH_URL};
use models::Page;
use reqwest::{Client, Error, StatusCode};

pub mod api;
pub mod consts;
pub mod models;

pub async fn fb_health_check() -> Result<StatusCode, Error> {
    let cl = Client::new();
    let resp = cl.get(FB_URL).send().await?;

    dbg!(&resp);
    Ok(resp.status())
}


pub async fn fb_get_me(key: String) -> Result<Page, Error> {
    let cl = Client::new();
    let resp = cl.get(format!("{}me?fields=id,name,followers_count&access_token={}", GRAPH_URL, key)).send().await?;
    dbg!(&resp);
    let page = resp.json::<Page>().await?;
    
    Ok(page)
}

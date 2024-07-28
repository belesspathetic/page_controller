use reqwest::{Client, Error, StatusCode};
use consts::{FB_URL, GRAPH_URL};

pub mod consts;
pub mod models;
pub mod api;

pub async fn fb_health_check() -> Result<StatusCode, Error> {
    let cl = Client::new();
    let resp = cl.get(FB_URL).send().await?;

    dbg!(&resp);
    Ok(resp.status())
}

pub async fn fb_get_me() -> Result<(), Error> {
    let cl = Client::new();
    let resp = cl.get(format!("{}me", GRAPH_URL)).send().await?;

    dbg!(resp);

    Ok(())
}




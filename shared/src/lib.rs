use reqwest::{Client, Error};

const BASE_URL: &str = "https://facebook.com";

pub async fn fb_health_check() -> Result<(), Error> {
    let cl = Client::new();
    let resp = cl.get(BASE_URL).send().await?;

    dbg!(resp);

    Ok(())
}



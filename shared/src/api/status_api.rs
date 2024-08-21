use crate::consts::BACK;
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use reqwest::Error;
use reqwest::Client;

pub async fn status_api(key: String) -> Result<String, Error> {
    let cl = Client::new();

    let endpoint = format!("http://{}/status", BACK);

    let resp = cl
        .get(endpoint)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .query(&[("key", key)])
        .send()
        .await?;

    // Получение текста ответа
    let body = resp.text().await?;
    eprintln!("Received response body: {}", body);
    let status: String = serde_json::from_str(&body).unwrap();

    Ok(status)

}
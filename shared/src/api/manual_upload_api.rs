use crate::consts::BACK;
use crate::models::ManualData;
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use reqwest::Error;
use reqwest::Client;

pub async fn manual_upload_api(data: ManualData) -> Result<(), Error> {
    let cl = Client::new();

    let endpoint = format!("http://{}/manual_upload", BACK);
    let json_body = serde_json::to_string(&data).unwrap();
    
    let _ = cl
        .post(endpoint)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_body)
        .send()
        .await?;

    Ok(())
}
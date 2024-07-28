use shared::consts::{FB_URL, GRAPH_URL};

#[tokio::test]
async fn get_fb() {
    let cl = reqwest::Client::new();
    let resp = cl.get(FB_URL).send().await.expect("failed to send reqwest");

    dbg!(&resp);
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn get_graph() {
    let cl = reqwest::Client::new();
    let resp = cl.get(format!("{}me", GRAPH_URL)).send().await.expect("failed to send reqwest");

    dbg!(&resp);
    assert_eq!(resp.status(), 400);
}

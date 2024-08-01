use back::run;
use reqwest::Client;
use std::net::TcpListener;
use tokio::task;

pub struct TestApp {
    pub address: String,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("127.0.0.1:{}", port);
    // dbg!(&address);

    let server = run(&address).expect("Failed to run test suite");

    // Spawn the server
    task::spawn(server);

    TestApp {
        address: format!("http://{}", address),
    }
}

#[tokio::test]
async fn own_health_check_works() {
    let app = spawn_app().await;
    let cl = Client::new();

    // Give the server some time to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let resp = cl
        .get(&app.address)
        .send()
        .await
        .expect("Failed to execute request");
    assert!(resp.status().is_success());
}

#[tokio::test]
async fn fb_health_check_works() {
    let app = spawn_app().await;
    let cl = Client::new();

    // Give the server some time to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Set endpoint
    let endpoint = format!("{}/fb_health_check", app.address);

    let resp = cl
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request");
    assert!(resp.status().is_success());
}

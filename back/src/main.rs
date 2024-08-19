use back::run;
use shared::consts::BACK;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    run(BACK)?.await
}

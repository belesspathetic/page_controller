use back::run;

const ADDRESS: &str = "127.0.0.1:5554";
#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(ADDRESS)?.await
}

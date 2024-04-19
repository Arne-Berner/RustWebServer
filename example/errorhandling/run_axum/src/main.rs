
#[tokio::main]
async fn main() {
    let config = "127.0.0.1:3001";
    errorhandling::run(config).await;
}

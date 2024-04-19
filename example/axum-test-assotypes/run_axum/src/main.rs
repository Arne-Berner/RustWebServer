#[tokio::main]
async fn main() {
    let config = "127.0.0.1:3000";
    axumtest::run(config).await;
}

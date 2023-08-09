use std::time::Duration;

use tokio::time::{sleep};


#[tokio::main]
async fn main() {
    let handler = axumtest::oneshot().await;
    sleep(Duration::from_secs(3)).await;
    handler.sender.send(1).unwrap();

    sleep(Duration::from_secs(20)).await;
}

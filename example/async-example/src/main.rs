use async_example::secondprint;
use async_example::AsyncCommandHandler;
use async_example::CustomError;
use async_example::Service;
use tokio::time::{sleep_until, Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let mut service = Service {};
    // join awaits multiple functions, so that they are running in parallel (if possible)
    let (_, res) = tokio::join!(secondprint(), service.handle(()));
    res
}

use std::future::IntoFuture;

use tokio::net::TcpListener;

use tokio::sync::oneshot::{Receiver, Sender};
pub mod controller;
pub mod service;

pub async fn run(config: &str) {
    // this would hold all the services and broker not just one
    // brokers are all structs abstracting away dependencies (like a logger, or the orm)
    // this would be a function returning the store, instead of the single service
    let store = service::ChangeCustomerService::new().await;
    let router = controller::setup_router(store).await;
    let listener = TcpListener::bind(config).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

pub struct OneShotHandler {
    pub sender: Sender<i32>,
}

pub async fn oneshot() -> OneShotHandler {
    let store = service::ChangeCustomerService::new().await;
    let router = controller::setup_router(store).await;
    let (tx, rx) = tokio::sync::oneshot::channel::<i32>();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let serve = axum::serve(listener, router).with_graceful_shutdown(rx_shutdown(rx));
    tokio::task::spawn(serve.into_future());

    OneShotHandler { sender: tx }
}

async fn rx_shutdown(rx: Receiver<i32>) {
    println!("before this works?");
    let res = rx.await;
    match res {
        Ok(_) => println!("shutting down gracefully"),
        Err(_) => println!("something weird happened"),
    }
}

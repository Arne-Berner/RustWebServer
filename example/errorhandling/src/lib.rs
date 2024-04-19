

pub mod service;
pub mod controller;

pub async fn run(config: &str){
    let store = service::GetCustomerService::new().await;
    let router = controller::setup_router(store).await;
    axum::Server::bind(&config.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
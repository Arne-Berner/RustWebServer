use axum::body::Body;
use axum::http::Request;
use axum::{
    extract::{self, Json, State},
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::service::{ChangeCustomerParamObj, CommandHandler, Customer};
use std::sync::Arc;

pub async fn setup_router<Service>(service: Service) -> Router
where
    Service: CommandHandler<ParamObject = ChangeCustomerParamObj, Result = String> + 'static,
{
    // this is a shared state object
    let services = Arc::new(tokio::sync::Mutex::new(service));

    let router = Router::new()
        .route("/", post(post_root))
        //.route("/int", post(post_int))
        .route("/int", post(handler))
        .route("/customer", get(customer))
        .route("/path/:userid", post(path))
        .route("/changeservice", post(change_service))
        .route("/request", get(request))
        // this is how you add the shared state
        .with_state(services);
    router
}

async fn request(body: Request<Body>) -> String {
    format!(
        "{} {:?} {} {:?} {:?}",
        body.uri(),
        body.headers(),
        body.method(),
        body.body(),
        body.version()
    )
}

async fn path(extract::Path(id): extract::Path<u32>) -> String {
    format!("{}", id)
}

async fn change_service<Service>(
    State(service): State<Arc<tokio::sync::Mutex<Service>>>,
    extract::Json(payload): extract::Json<Customer>,
) -> String
where
    Service: CommandHandler<ParamObject = ChangeCustomerParamObj, Result = String> + Send,
{
    let param_obj = ChangeCustomerParamObj::new_from_customer(payload);
    let res = service.lock().await.handle(param_obj).await.unwrap();
    println!("{:?}", res);
    res
}

//this is how you would return an explicit Json customer
async fn customer() -> Json<Customer> {
    axum::Json(Customer {
        firstname: "foo".to_string(),
        lastname: "bar".to_string(),
    })
}

#[derive(Debug, Deserialize)] // Deserialize the JSON payload into this struct
struct IntegerInput {
    integer_input: i32, // Assuming the input is an integer
}
// extractors look like the parameter here. This will automatically deserialized to String with axum
async fn post_int(extract::Json(payload): Json<IntegerInput>) -> String {
    let value = payload.integer_input;
    println!("{:?} is the number!", value);
    value.to_string()
}
// extractors look like the parameter here. This will automatically deserialized to String with axum
async fn post_root(Json(payload): Json<String>) -> String {
    payload
}

async fn handler(form: axum::extract::Form<IntegerInput>) -> () {
    // Extract the integer value from the form data
    let value = form.integer_input;

    // Process the integer value as needed (e.g., perform some computation)
    // Here, we'll simply return the received integer as a response
    println!("{:?} is the number!", value);
}

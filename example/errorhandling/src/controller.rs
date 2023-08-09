use axum::{
    extract::{Json, State},
    routing::{get},
    Router,
    http::StatusCode, response::{IntoResponse, Response},
};
use serde_json::json;


use crate::service::{GetCustomerService, CustomError};

pub async fn setup_router(service: GetCustomerService) -> Router
where
{
    // this is a shared state object
    // let services = Arc::new(tokio::sync::Mutex::new(service));

    let router = Router::new()
        .route("/", get(hello))
        .route("/first", get(getfirst))
        .route("/last", get(getlast))
        .with_state(service);
    router
}

async fn hello(){
    println!("this worked")
}

async fn getfirst(
    State(service): State<GetCustomerService>,
) -> Result<String, AppError>
{
    let res = service.call_my_name().await?;
    Ok(res.into())
}

async fn getlast(
    State(service): State<GetCustomerService>,
) -> Result<String, AppError>
{
    let res = service.call_my_lastname().await?;
    Ok(res.into())
}

// new type pattern
enum AppError {
    ServiceError(CustomError),
}

// if CustomError is given and AppError is wanted, 
// this will take the value of Custom Error and put it into AppError
impl From<CustomError> for AppError {
    fn from(value: CustomError) -> Self {
        AppError::ServiceError(value)
    }
}

// this will turn AppError into an Axum Error if returned
impl IntoResponse for AppError {
    // https://docs.rs/axum/latest/axum/response/index.html
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ServiceError(CustomError::InternalError) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Oops, something went wrong.")
            }
            AppError::ServiceError(CustomError::RepoError) => {
                (StatusCode::NOT_FOUND, "Could not find any user at all.")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        // into response only works on types that implement into_response_part
        (status, body).into_response()
    }
}

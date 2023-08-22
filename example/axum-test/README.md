### Serde
Anything that will be send as a return value or as a parameter must implement Deserialize/Serialize from serde. Careful: the derive feature flag must be set to use 
```rust
#[derive(Deserialize)]
```

### Axum walkthrough:
You can find all items for axum [here](https://docs.rs/axum/latest/axum/all.html) and an overview [here](https://docs.rs/axum/latest/axum/index.html)
In axum you will need to instantiate a new [Router](https://docs.rs/axum/latest/axum/struct.Router.html) on which you can place any routes that are important to you. This looks like this:
```rust
    let app = Router::new()
    .route("/", post(post_root))
        .route("/customer", get(customer).post(change_service))
        .route("/path/:userid", post(path))
        .route("/request", get(request))
        // this is how you add the shared state
```
The accepted methods for this are get, post and delete and can be added for a single route using the . operator.  
To actually run the server you will need to use
```rust
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
```
which takes the app from above.  
If you want to share state with the routes, you can do so by calling .with_state like so:

```rust
    let shared_state = Arc::new(Mutex::new(service));

    let app = Router::new()
    .route("/", post(post_root))
    .with_state(shared_state);
```
This will be called by [extractors](https://docs.rs/axum/latest/axum/extract/index.html). In our case with the commandhandler trait, an extractor with state could look like this:
```rust
async fn change_service<Service>(
    State(mut state): State<Arc<Mutex<Service>>>,
    extract::Json(payload): extract::Json<Customer>,
) -> String
where
    Service: CommandHandler<ChangeCustomerParamObj, String> + Send,
{
    let param_obj = ChangeCustomerParamObj::new_from_customer(payload);
    let res = state.lock().await.handle(param_obj).await.unwrap();
    println!("{:?}", res);
    res
}
```
This example also shows how any posted Json payload can be extracted from the request body. For this to work Customer needs to implement Deserialize (a serde Trait).

### How services and brokers are stored and set up
In the configure file, we have some service called "setup services" which is the composite root for all services. This gives back a vec<services> (or whatever is appropriate). This will be used in the router config file via method injection, where each router needs to be connected to the right service (can this step of choosing the right service for the router be abstracted away?). This would throw an compilation error if done wrong, because the handlers are requesting specific services via the generic like so: 
```rust
async fn change_service<Service>(
    State(mut service): State<Arc<tokio::sync::Mutex<Service>>>,
    extract::Json(payload): extract::Json<Customer>,
) -> String
where
    Service: CommandHandler<ChangeCustomerParamObj, String> + Send,
{
    let param_obj = ChangeCustomerParamObj::new_from_customer(payload);
    let res = service.lock().await.handle(param_obj).await.unwrap();
    println!("{:?}", res);
    res
}
```
The same holds true for broker(like tracing or the repository). Those services will be set up via using the commandhandler trait in the setup services, so that any service that requests another service gets that specific one. That way any service or handler does not need to know where the service comes from and it just needs to ask for the service it needs via the commandhandler abstraction. This way the repository broker for example will be invisible until called in the appropriate service.

### TODOs for the real thing
#### Extractors
Extractors can be wrapped into the Result type, so that you are able to respond with an appropriate errormessage.
```rust
async fn create_user(payload: Result<Json<Value>, JsonRejection>) {
    match payload {
        Ok(payload) => {
            // We got a valid JSON payload
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            // Request didn't have `Content-Type: application/json`
            // header
        }
        Err(JsonRejection::JsonDataError(_)) => {
            // Couldn't deserialize the body into the target type
        }
        Err(JsonRejection::JsonSyntaxError(_)) => {
            // Syntax error in the body
        }
        Err(JsonRejection::BytesRejection(_)) => {
            // Failed to extract the request body
        }
        Err(_) => {
            // `JsonRejection` is marked `#[non_exhaustive]` so match must
            // include a catch-all case.
        }
    }
}
```
#### Multiple Services/State
Since we do not want to provide all the services for all routes, we could make a store, that holds all the appropriate functions or we could provide each route with a service via closures like so (taken from [here](https://docs.rs/axum/latest/axum/index.html#sharing-state-with-handlers)):
```rust
struct AppState {
    // ...
}

let shared_state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route(
        "/users",
        post({
            let shared_state = Arc::clone(&shared_state);
            move |body| create_user(body, shared_state)
        }),
    )
    .route(
        "/users/:id",
        get({
            let shared_state = Arc::clone(&shared_state);
            move |path| get_user(path, shared_state)
        }),
    );

async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) {
    // ...
}

async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<AppState>) {
    // ...
}

#[derive(Deserialize)]
struct CreateUserPayload {
    // ...
}
```
#### Statuscode
Will we build a wrapper for the axum [Statuscode](https://docs.rs/http/0.2.9/http/status/index.html)?

### Problems:
Will we get deadlocks with the [tokio mutex?](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html) and how could we use RwLock instead for read and writes?


### feature flags
Since we want to be able to run this server in an integration test and use a different setting, we can use rusts build in feature flags. For more information see [this blogpost.](https://blog.frankel.ch/different-test-scopes-rust/) Another alternative is just using a thin main wrapper for production and integration tests and just run the integration binary for deployment checks.

### Curl tests
get requests to root look like this:  
```bash
curl -X GET -H "Content-Type: application/json" 127.0.0.1:3000/;
```
post requests look like this (in this case i post a customer):  
```bash
curl -X POST -H "Content-Type: application/json" --data '{"firstname": "bob", "lastname": "dylan"}' 127.0.0.1:3000/hello;
```

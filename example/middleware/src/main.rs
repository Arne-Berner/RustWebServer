use std::{future::Future, pin::Pin};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

trait Handler {
    type Future: Future<Output = Result<HttpResponse, Error>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future;
}

struct RequestHandler;
struct OurFuture<Ok>{
    // maybe as phantomdata??
    Okay: Ok
}

// could work, but not sure if it is send
impl<Ok> Future for OurFuture<Ok>{
    type Output = Result<Ok, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

impl Handler for RequestHandler {
    // We use `Pin<Box<...>>` here for simplicity, but could also define our
    // own `Future` type to avoid the overhead
    type Future = OurFuture<HttpResponse>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        Box::pin(async move {
            // same implementation as we had before
            if request.path() == "/" {
                Ok(HttpResponse::ok("Hello, World!"))
            } else if request.path() == "/important-data" {
                let some_data = fetch_data_from_database().await?;
                Ok(make_response(some_data))
            } else {
                Ok(HttpResponse::not_found())
            }
        })
    }
}
struct HttpResponse{}
struct HttpRequest{}
struct Error{}
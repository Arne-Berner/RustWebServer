### What are tower middlewares?
From [their docs](https://docs.rs/tower/latest/tower/):
Tower is a library of modular and reusable components for building robust networking clients and servers.


Tower provides a simple core abstraction, the Service trait, which represents an asynchronous function taking a request and returning either a response or an error. This abstraction can be used to model both clients and servers.

In short: We can use tower in axum as our middleware abstraction.

### How do you use tower middleware?
The [axum docs](https://docs.rs/axum/latest/axum/middleware/index.html) on middleware can give you an easy introduction to the topic. But the examples, especially the [tracing example](https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/src/main.rs) is very helpfull.

Tower itself has thorough [docs](https://docs.rs/tower/latest/tower/) but also very nice [guides](https://github.com/tower-rs/tower/tree/master/guides).

We could start implementing our own authentication middleware with Tower using [this guide](https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md).

### How do we implement middleware here?
This middleware will be using the tower structure, so that it can be build with the service builder and authenticate a user using paseto tokens and hashing and salting is done via the argon2 crate. An example for authentication is found [in the rust-web-dev github repo.](https://github.com/Rust-Web-Development/code/blob/785e6b1ccb94005d229877955365d868cc012494/ch_11/src/routes/authentication.rs). The github repo to [API security](https://github.com/NeilMadden/apisecurityinaction) is written in java, but can give us some pointers, what might be good middleware for our application. 

Combining the rust-web-dev example with the tower http [validate\_request trait](https://docs.rs/tower-http/latest/tower_http/validate_request/index.html) could be the solution we are looking for. This could also work with jwt easily.
Maybe the [AsyncAuthorizeRequest trait](https://docs.rs/tower-http/latest/tower_http/auth/trait.AsyncAuthorizeRequest.html) is even more fitting.

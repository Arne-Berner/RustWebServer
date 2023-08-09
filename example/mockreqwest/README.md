### What is this for?
This server is used for integration testing. You should be able to simply use this lib in your application to run the requests, that are needed. Every route should have at least it's own integration test for the positive path and one common negative path.

### how do you specify which tests to run?
https://doc.rust-lang.org/cargo/commands/cargo-test.html
this feature flag could help too, that way we could only run the integration feature when we do a pullrequest or deploy our code.
https://blog.frankel.ch/different-test-scopes-rust/
But this would pollute our code with weird integration tests, that are just sitting in there. Maybe it would be better to just use a library, that will test the api given in the system.

#### Alternative
We will place our businiesslogik in libraries (we do so anyway), then we could reuse those libraries in an "integrationtest" project. This would have a small main bin (just like in our main project) and starts up the libraries just like in our main server. But it also has a lib with reqwest, which will send requests to the http server. If this fails the cicd will not accept the pullrequest.
[this](https://github.com/programatik29/axum-server/blob/master/examples/graceful_shutdown.rs) is an example how you can shut down an axum server from a different lib after using it. The [warp example](https://github.com/Rust-Web-Development/code/blob/785e6b1ccb94005d229877955365d868cc012494/ch_11/src/lib.rs#L127) is directly taken out of the manning book.

### What is this oneshot server?
You can start up our server and get a channel (a one way road, that works in an async environment) to our server. With this you can send (you have the sender) the shutdown command to our server, which stops the server from running in a controlled manner.

### Unwinding and futures util
"In programming, unwinding means to remove the functions and variables from the stack in reverse order, to leave a clean slate in case of an exception. If we expect a panic from a function call, we can use the catch_unwind call from the Rust standard library (http://mng.bz/K0dj). In our case, we use the wrapper AssertUnwindSafe (http://mng.bz/9VJ7), which signals that the variables we use are safe to unwind. We wrap our function call with it, and call catch_unwind (http://mng.bz/jA9r) from the futures_util crate, which “catches unwinding panics while polling the future.”

If the register_new_user function panics (since we don’t handle errors and unwrap them), and if something else goes wrong during our testing, we unwind the stack, stop the mock server, and end the process." - rust web dev book.
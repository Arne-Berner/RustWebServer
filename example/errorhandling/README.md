### What is errorhandling?
There will be different errors depending on the layer we are in. The businesslayer should not know about the specific errors in the repository and the controller. But the controller will need to reinterpret the errors from the business layer into errors, that are from a dependency. So it needs to implement from<externalType> to<otherExternalType>. 

[Official axum docs](https://docs.rs/axum/latest/axum/error_handling/index.html)

### How do we want to do this?
Since both types (the CustomError and the Response in this case) are external, we need to wrap CustomError into a new Type (called AppError here). That way we can implement IntoResponse for the external type.

### Why do we not use the errors from axum in our businesslayer?
The rust ecosystem is ever changing, so being dependant on axum might hinder growth in the future.

### Where do I find references to good error handling?
Rust for Rustaceans has a whole chapter on it, but [this example](https://github.com/tokio-rs/axum/blob/main/examples/error-handling-and-dependency-injection/src/main.rs) is about axum specifically.


### How to use this example:
Open a command window, go to errorhandling/run\_axum and type
```bash
cargo run
```
open another command window and type
```bash
curl -X GET -H "Content-Type: application/json" 127.0.0.1:3001/first; 
curl -X GET -H "Content-Type: application/json" 127.0.0.1:3001/last; 
```

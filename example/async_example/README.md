## async in Rust
Async in rust works with the keyword "async" and "await". Async is used to make the function async. This will change the return type to a future of that return type. Await is used to fullfill that future. Async futures are lazy in rust, so they need to be awaited. Awaiting in this context means either fullfilling that future or yielding the thread for other await functions above this in the callstack to make progress. this can be seen in main with the "join" between two async functions.
An in depth practical explanation can be found [here](https://www.youtube.com/watch?v=ThjvMReOXYM)
### awaiting a function
Awaiting a function will poll the future in the background, until they are finished. This will happen in blocks of code until 
### async traits
Async Traits are not supported in Rust yet. But there is a nightly version of async traits. That said, there are some issues that will probably not cause any issues for us. First and foremost: using nightly means working on a development stage of a feature. But async traits probably wont change much. 
If you try to spawn from a generic function like print_all, and (like the majority of async users) you use a work stealing executor that requires spawned tasks to be Send, you'll hit an error which is not easily resolved. So this wouldnt work:
```rust
fn spawn_print_all<I: AsyncIterator + Send + 'static>(mut count: I)
where
    I::Item: Display,
{
    tokio::spawn(async move {
        //       ^^^^^^^^^^^^
        // ERROR: future cannot be sent between threads safely
        while let Some(x) = count.next().await {
            //              ^^^^^^^^^^^^
            // note: future is not `Send` as it awaits another future which is not `Send`
            println!("{x}");
        }
    });
}
```

#### async-trait crate
This uses a Pin<Box<dyn TYPE>> and is therefore heap allocated. But since we would mostly use it in a Dependency injection context, that might not be an issue. 

#### nightly
To get the async trait feature, the project must be set to nightly. This can be done via 
```bash
rustup default nightly
```
or by setting the rust-toolchain.toml as seen in this project.

### nightly vs async traits
Should we use nightly or async traits? Async traits are supposed to be ready but they use pin box, which is a runtime feature, while the nightly version uses impl Future. Nightly sounds better, but has some security implications due to using nightly.
The problem with nightly is, that it does not work well with generic traits and send. So it is not possible to send it over threads for generic types, when you use dependency injection. But this is done extensively in Tokio (and other frameworks) to enable load balancing for async routines. It is best explained in [this issue](https://github.com/rust-lang/rust/issues/103854). 
#### Update
There are some news on this issue which are explained [here](https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html). MVP Part 2 shows a workaround in [this playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=2066934a05cb9eafc0b47af7bdf8c57f). 

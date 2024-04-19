## What is this repo?
This repo is a playfield for trying out webserver stuff in rust.

### The most important examples
- axum-test-assotypes (will start axum and uses the commandhandler pattern)
- mockreqwest (for integration tests)
- errorhandling (for handling errors)
- repo (for handling the db)
- sqlx (for working with postgres)


### Examples
You will find all the necessary pieces to build the backend in the examples folder. If you are unsure about a certain component you can play around with this component, without the need to understand it's context.
Each example has a readme file that will give you some pointers about how you implement each piece of code. 
Most readmes will have one or more sections describing "What is this example", "How is it implemented here" and "Why certain decisions are made".

## Books
### Rustservers und service apps published by Manning 
[This book](https://livebook.manning.com/book/rust-servers-services-and-apps?origin=dashboard) has some nice ideas about errorhandling and could be a nice tutorial for anyone trying to build a fast frontend with a rust backend.

### rust web develeopment published by Manning
[Rust web development](https://livebook.manning.com/book/rust-web-development/) on the other hand will dive a little bit deeper into the workings of a nice rust server. There are some notes about good api design, but also about different frameworks. It uses a simple framework to show you the most important parts of an API. Including integration tests and mockserver. 

### zero to production by Luca Palmieri
[Zero to Production](https://www.zero2prod.com/) is an amazing book. You can read the epub version from front to back without any layout issues and it's highly educational. It starts with an idea and goes through several iterations until you have a working application. Not only an application, but an application distributed via CI/CD and highly scalable.

## Frameworks
### Axum
Axum is a thin layer on top of the Hyper crate, which is frequently used for HTTP Requests. It is maintained by the creators of Tokio and has more downloads than Actix (despite it's young age).
### Warp
Warp also builds on top of hyper and is the oldest framework of the three frameworks discussed here. It's a bit slower but has the most "experienced" community.
### Actix
Actix is a lightweight Rust Webframework with the best speed benchmarks, but roundabout the same capabilities as the two above. The downside to actix is its syntax and that it's harder to expand than axum.
### A comparism from 2022
https://kerkour.com/rust-web-framework-2022

## DB
### SQLx
SQLx is a SQL Toolkit written in rust. It features async, migrations and is compile time checked ([without being an ORM!](https://github.com/launchbadge/sqlx#sqlx-is-not-an-orm).
### SeaORM
SeaORM is build on SQLx and promises the same DSL (domain specific language) driven, no worries approach most ORMs do. But it's still missing a lot of features to be convenient and it's so young, that you might need to learn a DSL for nothing.
### Diesel
Diesel is not asynchronous and this might lead to some problems down the road, since async environments are not used to blocking IO that takes too much time. Although it is very stable


## What is still missing?

### Tracing example
Will you use [tracing?](https://crates.io/crates/tracing)?
If so, you might want to have an example using it. Tracing is relative light weight and easy to use. Many other frameworks and code examples use it right now. (may 2023)

### Benchmarking example
Benchmarking is mostly used with [Criterion](https://crates.io/crates/criterion), which [might be part of rust soon.](https://blog.rust-lang.org/inside-rust/2022/04/04/lang-roadmap-2024.html?ref=notamonadtutorial.com). This [Criterion example](https://github.com/bheisler/criterion.rs/blob/master/book/src/getting_started.md) might get us started.

### Configuration
An example on how to insert configurations in a secure matter into the application would be nice.

### Writing Tests for the examples
Instead of reading the README, it should be possible to learn how the application or example work by reading the tests.

### Everything combined in this folder
This folder will hold the actual example code at some point with the minimum needed. It will not include real services you will use or real data for the database.

### Rustbooks
[The OG Rustbook](https://doc.rust-lang.org/book/)  
[Rust by Example](https://doc.rust-lang.org/rust-by-example/)  
[Rustonomicon](https://doc.rust-lang.org/nomicon/)  
[Cargobook](https://doc.rust-lang.org/cargo/)  
[Performance Benchmarking](https://nnethercote.github.io/perf-book/benchmarking.html)  
[Clippy book](https://doc.rust-lang.org/nightly/clippy/index.html)  
[Many more](https://www.rust-lang.org/learn)  


### Questions:
### Is it possible to separate async tests from an extern runtime? 
No. In this case the tests need to be run in Tokio or you'd need to use something like the futures crate to create a simple async runtime (not adviced).

### Is it possible to separate the businesslogic from tokio? 
Yes. The Businesslogic could be inside a lib or just not using an async main. You cannot however write async tests as stated above.  

### Do you really need to use the async trait crate?
No. There is a way to use async traits in the stable version, although you need to pull in "trait-variant" now.

### Can you use pure DI with rust? 
Yes. Traits can be used in a similar fashion as interfaces.
But the "typesafe" way to do this would be with generics. Using dyn traits as Trait objects is supposingly an antipattern, although it's basically the same as any use of an interface in an OOP language.

### What do you want to log?
Logging too much can be a codesmell.

### Do you want to write a "test layer" that runs all the tests?
This would not be very idiomatic for rust, but you could prevent tokio leaking into our main layer. Although that would just be pushing the problem to another place. Besides this minor advantage, it would make you only test observable behaviour, which is good. But it takes time to setup properly and would run the tests far away from the actual code. So maintaining it could take a bit longer. Also you would not be able to see what the code is supposed to do, without going to that specific folder. Lastly - doctests would be meaningless in a separate layer.

Possible solution: Writing tests in "mod tests" in the corresponding files. That way you could implement tokio for that mod only and use super::\*; the tests would still be coupled to tokio, but the rest could work without it. Also you would only test public functionality that way.

## What is this repo?
This repo is a playfield for trying out webserver stuff in rust.

### Examples
You will find all the necessary pieces to build the backend in the examples folder. If you are unsure about a certain component you can play around with this component, without the need to understand it's context.
Each example has a readme file that will give you some pointers about how we implement each piece of code. 
Most readmes will have one or more sections describing "What is this example", "How is it implemented here" and "Why certain decisions are made".

## Books
### Rustservers und service apps von Manning 
[This book](https://livebook.manning.com/book/rust-servers-services-and-apps?origin=dashboard) has some nice ideas about errorhandling and could be a nice tutorial for anyone trying to build a webservice with both - a rust backend and a rust webassembly frontend.

### rust web develeopment von Manning
[Rust web development](https://livebook.manning.com/book/rust-web-development/) on the other hand will dive a little bit deeper into the workings of a nice rust server. There are some notes about good api design, but also about different frameworks. It uses a simple framework to show you the most important parts of an API. Including integration tests and mockserver. 

## Frameworks
### Axum
Axum ist ein kleiner Layer auf dem Hyper Crate, welcher für HTTP Requests zuständig ist. Es wird von den machern von Tokio maintained und hat mittlerweile mehr Downloads als Actix (trotz des jungen alters).
### Warp
Warp baut auf Hyper auf und ist das älteste Framework der drei. Es ist etwas langsamer, hat aber die "erfahrendste" Community(mit dem Framework).
### Actix
Actix ist ein lightweight Rust Webframework, welches HTTP Requests, aber auch JWT authentication und ähnliches ermöglicht. So kann man mit actix schnell ein Backend Prototypen.  
Actix benötigt immer einen HTTP Webserver, in welchem eine Actix App läuft. Diese App repräsentiert die Routes, die erreicht werden können. Unter den Routes sind jeweils handler zu finden, welche beim aufrufen der Route ausgeführt werden und einen Response erzeugen.
### Ein Vergleich aus 2022
https://kerkour.com/rust-web-framework-2022

## What is still missing?

### SeaORM example
SeaORM is an async ORM that has some nice capabilities. The other big orm in rust would be diesel. But it has no dynamic queues and is synchronous.

### Trace example
Will we use [tracing?](https://crates.io/crates/tracing)
If so, we might want to have an example using it. Tracing is relative light weight and easy to use. Many other frameworks and code examples use it right now. (may 2023)

### Benchmarking example
Benchmarking is mostly used with [Criterion](https://crates.io/crates/criterion), which [might be part of rust soon.](https://blog.rust-lang.org/inside-rust/2022/04/04/lang-roadmap-2024.html?ref=notamonadtutorial.com)

### Everything combined in this folder
This folder will hold the actual example code with the minimum needed. It will not include real services we will use or real data for the database.

### Rustbooks
[The OG Rustbook](https://doc.rust-lang.org/book/)
[Rust by Example](https://doc.rust-lang.org/rust-by-example/)
[Rustonomicon](https://doc.rust-lang.org/nomicon/)
[Cargobook](https://doc.rust-lang.org/cargo/)
[Performance Benchmarking](https://nnethercote.github.io/perf-book/benchmarking.html)
[Clippy book](https://doc.rust-lang.org/nightly/clippy/index.html)
[Many more](https://www.rust-lang.org/learn)


### Fragen:
### Is it possible to separate async tests from tokio? 
No.

### Is it possible to separate the businesslogic from tokio? 
Yes. The Businesslogic could be inside a lib or just not using an async main. You cannot however write async tests as stated above.  

### Do we really need to use the async trait crate?
Yes. Altough there should be async traits coming to rust at some point, but the nightly version does not work the way we would want to use it yet, so we will have to use the lib. 

### Can we use pure DI with rust? 
Yes. Traits can be used in a similar fashion as interfaces.
Kann man mit Rust pure DI ausüben? Ist es also Möglich die einzelnen Services (oder in Rust Mods) Dezentral zu erstellen und so den Object Graph zentral zu halten und die (externen und internen) Dependencies über Traitobjects zu injezieren?  Antwort: Ja

### What do we want to log?
Logging too much can be a codesmell.

### Do we want to write a "test layer" that runs all the tests?
This would not be very idiomatic for rust, but we could prevent tokio leaking into our main layer. Although that would just be pushing the problem to another place. Besides this minor advantage, it would make you only test observable behaviour, which is good. But it takes time to setup properly and would run the tests far away from the actual code. So maintaining it could take a bit longer. Also you would not be able to see what the code is supposed to do, without going to that specific folder. Lastly - doctests would be meaningless in a separate layer.

Possible solution: Writing tests in "mod tests" in the corresponding files. That way we could implement tokio for that mod only and use super::\*; the tests would still be coupled to tokio, but the rest could work without it. Also we would only test public functionality that way.

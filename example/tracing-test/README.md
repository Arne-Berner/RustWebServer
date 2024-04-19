# The goal:
I want to add a functional example that will work nicely with tracing.  
It should highlight the use of "thiserror" and "anyError" (to an extend).  
In order to see the actual impact of tracing I think it is necessary to have a route, a service doing something with that route and a repository persisting data, that was send via that route. This whole process should be logged.

## SRC
Startup should be the place, where the application starts, the pgpool will be initiated and the services will get registered to the dependency container. This is also where lib will live, which will pub mod any necessary module main.rs will use. Main.rs is usually also used to setup any configuration.

## Tests
This is the place all integration tests will go

## Routes
Routes will handle how each route of the service will be handled.

## Repository
In e.g. you'll use sqlx to CRUD data to and from the db here.

## Domain
This is where the services with the domain logic lies. Everything will depend on this. This is also where the repository trait will be defined.


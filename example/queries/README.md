### CQRS (Command Query Responsibility Segregation)
Most Crud functions can be separated into Queries and Commands. Commands take in some parameters and have no return value, e.g. Create functions. Queries take in a search parameter and have a result, e.g. Read functions.

### What is this about?
This example is about a generic Query function and its uses in a business application environment. Each Query function will return the wanted entity and they can be composed in the compose root.

### Orchestration services
Clinicquery is a pure orchestration service. It has no functions except handle by itself and it's only function is to call the other two query functions.
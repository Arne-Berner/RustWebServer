### What is this example for?
This example shows how we will be able to separate our business logic from the data layer and the controller. Any changes to the access layer or the datalayer will not affect the business layer.

### Why do we need this?
The controller depends on the business layer, but the business layer does not need to know about the controller anyway. So there is no need for dependency inversion here. But the businesslayer will need to call the datalayer. 
### How do we achieve this?
To be able to make any changes to the datalayer without breaking the business layer we are using "generics" with "trait bounds". The repository itself uses associated types since there will be only one implementation of repository for each type. The type can be declared in our business logik. That way the datalayer will depend on the business layer for the repository trait and the entity, and not the other way around.  
The concrete implementation of the generics will be in the root composer, so that the businesslayer does not need to know about it. The businesslayer only needs to know that it wants some repository injected into it's constructor.

### Do we need the repo to be generic?
As soon as we want our businesslogik not to be the one implementing or depending on the data layer we need to use a solution like this. Instead of using generics you could also use "Trait Objects", but this would make this version not typesafe. This can be seen with the "NonDebugRepo" struct.
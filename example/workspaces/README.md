### Folder structure:
Every Folder contains a mod.rs, which in turn contains the named submodule. That way the files are searchable and the tabs are not all called "mod.rs".

### Explanation:
We have the composite root, which is the starting point for the project. This is the place where every service and controller gets constructed. This is also the place where configurations take place.  
Then there is the trait for commandhandler, which is explained in the commandhandler example. Our addClinicService implements the Trait, so that the controller later on can get a reference to that service via constructor injection and just call "handle" on it. The Controller also asks for a specific service using the param object, so that no other service will be registered by mistake. The Controller calls the addclinicservice, which will do the processing necessary to add the clinic (or anything like tracing could be used as a decorator for that service) and after processing is done, addclinicservice will call the clinic repository add method. The clinic repository implements the abstract "repository" itself (see the repository example). If in any of those stages an error occurs, it will be handled accordingly and then bubbled upwards to the controller. The Controller would put the error message into an appropriate error code and send this via https to the client.

### TODO add links to other examples


### Questions
Where and when should I use my Statuscodes? Should they be put in the Error right away?
### What is Commandhandler
Even though the name might suggest it, it is not the Command pattern. The name stems from the CQRS (Command Query Responsibility Segregation pattern). The idea behind the Commandhandler trait is that it will be an abstraction for any command that might be executed. The "Tower" crate with it's service resembles this structure much more.

### What is it's structure?
The Commandhandler is a trait, that takes in a generic "parameter object" as it's argument and uses the Result type (in this rust implementation) as a return type. Even though I say generic, what I mean is it has an associated type, that will differ from each struct implementing this trait. The response is - once again - a generic result<T, Error> and could be an ID, a Statuscode 200 or anything similar. The error will be a custom error, which will be implemented like in [this blogpost](https://fettblog.eu/rust-enums-wrapping-errors/).

### Why a parameter Object?
Using a parameter object we do not need to worry about the amount of parameters a function takes. But it also ensures that only services that can use that parameter object will call this function, because at the time the generics get their concrete type, the compiler will show you an error message, if the composition went wrong.

### How is it used here?
This program consists of the commandhandler trait, a service struct and two decorator structs. Those three structs implement the trait and are constructed in main. The DecoratorDecorator takes in the Decorator as an argument (but could take in any struct implementing commandhandler) and the Decorator takes in the service. That way only one call to handle will execute all three handle functions(one for each struct).

### What are the advantages/disadvantages?
The advantages are the following:
- any new function is completly decoupled
- generic decorators can be used for any function impl the trait
- due to the generics, it will benefit from compile time checking

The disadvantages are the following:
- it is verbose, because a service and a param obj needs to be created

### What is the difference to Queries?
This Question needs to be answered and it might be that there is no difference in our case. Even though there might be differences if a &mut self is needed, we should postpone that question until the need arrives.

### Why mut self?
&mut self will be needed as soon as a service has fields that will be changed with that service. So if there is a service that counts the number of calls to that service, it will need the &mut self.
This will also be the case, when using a repository, because as soon as we call a function that will have a &mut self, this function will collapse without it.

### What is the cs file?
That is the original c# implementation.

### Why a paramter Object?
Using a parameter object we do not need to worry about the amount of parameters a function takes. But it also ensures that only services that can use that parameter object will call this function, because at the time the generics get their concrete type, the compiler will show you an error message, if the composition went wrong.

### How is it used here?
This program consists of the commandhandler trait, a service struct and two decorator structs. Those three structs implement the trait and are constructed in main. The DecoratorDecorator takes in the Decorator as an argument (but could take in any struct implementing commandhandler) and the Decorator takes in the service. That way only one call to handle will execute all three handle functions(one for each struct).

### What are the advantages/disadvantages?
The advantages are the following:
- any new function is completly decoupled
- generic decorators can be used for any function impl the trait
- due to the generics, it will benefit from compile time checking

The disadvantages are the following:
- it is verbose, because a service and a param obj needs to be created

### What is the difference to Queries?
This Question needs to be answered and it might be that there is no difference in our case. Even though there might be differences if a &mut self is needed, we should postpone that question until the need arrives.

### Why mut self?
&mut self will be needed as soon as a service has fields that will be changed with that service. So if there is a service that counts the number of calls to that service, it will need the &mut self.
This will also be the case, when using a repository, because as soon as we call a function that will have a &mut self, this function will collapse without it.

### What is the cs file?
That is the original c# implementation.

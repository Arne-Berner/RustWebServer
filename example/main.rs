// this will construct the other layers
fn main() {
// an example customer
let customer = businesslayer::Customer{"Mary", "Doe"}
// constructing the services via pure DI
let accessservice = accessservice::new(addcustomerservice::new(customer_repository::new()))
}


mod accesslayer {

// this could be a http controller in a real application
// it does not need an abstraction in this case, because no other service depends on it (yet)
pub struct accessservice {
    addcustomerservice: businesslayer::AddCustomerService
}

impl accessservice{
    fn new(addcustomerservice: businesslayer::AddCustomerService) -> &Self{
        accessservice{ addcustomerservice}
    }
    fn addcustomer(firstname: &str, lastname: &str) -> Result{
        let addcustomer = businesslayer::AddCustomer{
            businesslayer::Customer{firstname, lastname}
        }

        let result = addcustomerservice.execute(addcustomer);
        return result;
    }

}

// this will be the layer that gets changed the least and everything should rely on
mod businesslayer {
// later implementation will be something like this
// pub trait IRepository{
//    fn read(&self) -> IEntity;}

// this will be used as an example
struct Customer { 
    firstname: &str,
    lastname: &str
}

// defines icommand abstraction
// it takes a param object and returns a simple Ok or error
pub trait ICommand<TCommand>{
    fn execute(command: TCommand) -> Result;
}

// concrete implementation of the Icommandservice
pub struct AddCustomerService{
    repository: IRepository
}

//param object for the execute method
struct AddCustomer{
    customer: Customer
}

// new method with abstract repository
impl AddCustomerService{
    pub fn new(repository: IRepository) -> Self {
        AddCustomerService { repository }
    }
}

// implementing the concrete icommand execute method
impl <TCommand: AddCustomer> ICommand for AddCustomerService{
    pub fn execute(customer: AddCustomer) -> Result{
        &self.repository.add(customer.customer)
    }
}

// this needs to be implemented in this mod, for Inversion of Control
pub trait IRepository{
    pub fn read(&self) -> Customer;
    pub fn add(Customer) -> Result;
}
}


// this could be a postgres database in a real application
mod datalayer {
// this is the concrete implementation of IRepository. 
// For this example i will not use a vec to keep it simple
pub struct customer_repository{
    customer: businesslayer::Customer
}

impl customer_repository{
    fn new(customer: businesslayer::Customer) -> &Self(
        customer_repository{customer})}

impl IRepository for customer_repository {
    fn read(&self) -> businesslayer::Customer{
        &self.customer.clone()}
    fn add(addedcustomer: businesslayer::Customer) -> Result{
        &self.customer = addedcustomer;
        return Ok();
    }
}



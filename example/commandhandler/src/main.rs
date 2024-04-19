fn main() {
    let param_obj = ChangeCustomerParamObj::new(format!("John"), format!("Smith"));
    let service = ChangeCustomerService::new();
   // match service.handle(param_obj) {
   //     Ok(_) => print!("It worked!"),
   //     Err(_) => print!("It ... also kind of worked. When you want it to be an error at least."),
   // }
    // let service = RandomService{};
    let deco_service = Logging::new(service);
    let mut deco_deco_service = Benchmarking::new(deco_service);
    match deco_deco_service.handle(param_obj) {
        Ok(_) => print!("It worked!"),
        Err(_) => print!("It ... also kind of worked. When you want it to be an error at least."),
    }
}

pub trait CommandHandler {
    type ParamObject;
    type ReturnType;

    fn handle(&mut self, param_obj: Self::ParamObject) -> Result<Self::ReturnType, CustomError>;
}

pub struct CustomError {}



// this does not work alone, because it does not implement command
// pub struct RandomService {}

pub struct ChangeCustomerService {
    customer: Customer,
}

impl ChangeCustomerService {
    pub fn new() -> Self {
        ChangeCustomerService {
            customer: Customer {
                firstname: "Mary".to_string(),
                lastname: "Doe".to_string(),
            },
        }
    }
    pub fn call_my_name(&self) {
        print!("\n{}\n", &self.customer.firstname);
    }
}

impl CommandHandler for ChangeCustomerService {
    type ParamObject = ChangeCustomerParamObj;
    type ReturnType = ();

    fn handle(&mut self, param_object: ChangeCustomerParamObj) -> Result<(), CustomError> {
        self.customer = param_object.customer;
        self.call_my_name();
        Ok(())
    }
}

#[derive(Debug)]
pub struct Customer {
    firstname: String,
    lastname: String,
}

#[derive(Debug)]
pub struct ChangeCustomerParamObj {
    customer: Customer,
}

impl ChangeCustomerParamObj {
    pub fn new(firstname: String, lastname: String) -> Self {
        ChangeCustomerParamObj {
            customer: Customer {
                firstname,
                lastname,
            },
        }
    }
}

pub struct Logging<Service> {
    service: Service,
}

impl<Service> Logging<Service> {
    pub fn new(service: Service) -> Self {
        Logging { service }
    }
}

impl<Service> CommandHandler for Logging<Service>
where
    Service: CommandHandler,
    Service::ParamObject: std::fmt::Debug,
{
    type ParamObject = Service::ParamObject;
    type ReturnType = Service::ReturnType;
    fn handle(
        &mut self,
        param_object: Service::ParamObject,
    ) -> Result<Service::ReturnType, CustomError> {
        //decorator function
        print!("This is before service: {:?}", param_object);
        let res = self.service.handle(param_object);
        print!("This is after service!");
        res
    }
}

pub struct Benchmarking<Service> {
    service: Service,
}

impl<Service> Benchmarking<Service> {
    pub fn new(service: Service) -> Self {
        Benchmarking { service }
    }
}

impl<Service> CommandHandler for Benchmarking<Service>
where
    Service: CommandHandler,
{
    type ParamObject = Service::ParamObject;
    type ReturnType = Service::ReturnType;
    fn handle(
        &mut self,
        param_object: Service::ParamObject,
    ) -> Result<Service::ReturnType, CustomError> {
        println!("This is before decor:");
        let res = self.service.handle(param_object);
        println!("\nThis is after decor!");
        res
    }
}

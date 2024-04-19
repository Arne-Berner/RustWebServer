use serde::{Deserialize, Serialize};

#[trait_variant::make(CommandHandler: Send)]
pub trait LocalCommandHandler: Clone + Send {
    type ParamObject;
    type Result;
    async fn handle(&mut self, param_obj: Self::ParamObject) -> Result<Self::Result, CustomError>;
}

#[derive(Debug, Clone)]
pub struct CustomError {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Customer {
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Clone, Deserialize)]
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
    pub fn new_from_customer(customer: Customer) -> Self {
        ChangeCustomerParamObj { customer }
    }
}

// this does not work alone, because it does not implement command
// pub struct RandomService {}

#[derive(Clone)]
pub struct ChangeCustomerService {
    customer: Customer,
}

impl ChangeCustomerService {
    pub async fn new() -> Self {
        ChangeCustomerService {
            customer: Customer {
                firstname: "Mary".to_string(),
                lastname: "Doe".to_string(),
            },
        }
    }
    pub async fn call_my_name(&self) {
        print!("\n{}\n", &self.customer.firstname);
    }
    pub async fn give_name(&self) -> String
    where
        String: Send,
    {
        format!(
            "Firstname: {}, Lastname: {}",
            self.customer.firstname, self.customer.lastname
        )
    }
}

impl CommandHandler for ChangeCustomerService {
    type ParamObject = ChangeCustomerParamObj;
    type Result = String;
    async fn handle(
        &mut self,
        param_object: ChangeCustomerParamObj,
    ) -> Result<Self::Result, CustomError> {
        self.customer = param_object.customer;
        self.call_my_name().await;
        Ok(self.give_name().await)
    }
}

// pub struct Decorator<Service> {
//     service: Service,
// }

// impl<Service> Decorator<Service> {
//     pub fn new(service: Service) -> Self {
//         Decorator { service }
//     }
// }

// impl<ParamObj, Service, Res> CommandHandler<ParamObj, Res> for Decorator<Service>
// where
//     Service: CommandHandler<ParamObj, Res>,
//     ParamObj: std::fmt::Debug,
// {
//     async fn handle(&mut self, param_object: ParamObj) -> Result<Res, CustomError> {
//         //decorator function
//         print!("This is before service: {:?}", param_object);
//         let res = self.service.handle(param_object).await;
//         print!("This is after service!");
//         res
//     }
// }

// pub struct DecoratorDecorator<Service> {
//     service: Service,
// }

// impl<Service> DecoratorDecorator<Service> {
//     pub fn new(service: Service) -> Self {
//         DecoratorDecorator { service }
//     }
// }

// impl<ParamObj, Service, Res> CommandHandler<ParamObj, Res> for DecoratorDecorator<Service>
// where
//     Service: CommandHandler<ParamObj, Res>,
// {
//     async fn handle(&mut self, param_object: ParamObj) -> Result<Res, CustomError> {
//         println!("This is before decor:");
//         let res = self.service.handle(param_object).await;
//         println!("\nThis is after decor!");
//         res
//     }
// }

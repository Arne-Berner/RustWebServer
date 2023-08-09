use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GetCustomerService{
    customer: Customer,
}

impl GetCustomerService {
    pub async fn new() -> Self {
        GetCustomerService{ customer: Customer {
            firstname: format!("Alice"),
            lastname: format!("Whothefuck"),
    }}
}
     pub async fn call_my_name(&self) -> Result<String, CustomError> {
        print!("\n{}\n", &self.customer.firstname);
        Err(CustomError::RepoError)
    }
     pub async fn call_my_lastname(&self) -> Result<String, CustomError> {
        print!("\n{}\n", &self.customer.lastname);
        Err(CustomError::InternalError)
    }
}

#[derive(Debug, Clone)]
pub enum CustomError {
    RepoError,
    InternalError
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Customer {
    pub firstname: String,
    pub lastname: String,
}
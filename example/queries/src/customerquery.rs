use crate::queryhandler::*;

#[derive(Debug, Clone)]
pub struct Customer {
    id: u32,
    firstname: String,
    lastname: String,
}

impl Customer {
    pub fn new(id: u32, firstname: String, lastname: String) -> Self {
        Customer { id, firstname, lastname}
    }
}

#[derive(Debug)]
pub struct SearchCustomerParamObj {
    id: u32,
}

impl SearchCustomerParamObj {
    pub fn new(id: u32) -> Self {
        SearchCustomerParamObj { id }
    }
}

pub struct SearchCustomerService {
    pub customers: Vec<Customer>
}

impl QueryHandler<SearchCustomerParamObj, Customer> for SearchCustomerService {
    fn handle(&self, param_object: SearchCustomerParamObj) -> Result<Customer, CustomError> {
        let found_customer = self.customers.iter().find(|customer| customer.id == param_object.id);

        match found_customer {
            Some(customer) =>Ok(customer.clone()),
            None => Err(CustomError::Internal),
        }
    }
}
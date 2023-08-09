use crate::{
    customerquery::{Customer, SearchCustomerParamObj},
    opquery::{Op, SearchOpParamObj},
    queryhandler::*,
};

#[derive(Debug, Clone)]
pub struct Clinic {
    op: Op,
    customer: Customer,
}

impl Clinic {
    pub fn new(op: Op, customer: Customer) -> Self {
        Clinic {op, customer}
    }
}

#[derive(Debug)]
pub struct SearchClinicParamObj {
    op_param_obj: SearchOpParamObj,
    customer_param_obj: SearchCustomerParamObj,
}

impl SearchClinicParamObj {
    pub fn new(op_id: u32, customer_id: u32) -> Self {
        let op_param_obj = SearchOpParamObj::new(op_id);
        let customer_param_obj = SearchCustomerParamObj::new(customer_id);

        SearchClinicParamObj { op_param_obj, customer_param_obj }
    }
}

pub struct SearchClinicService<OpQuery, CustomerQuery> {
    op_query: OpQuery,
    customer_query: CustomerQuery,
}

impl<OpQuery, CustomerQuery> SearchClinicService<OpQuery, CustomerQuery>
where
    OpQuery: QueryHandler<SearchOpParamObj, Op>,
    CustomerQuery: QueryHandler<SearchCustomerParamObj, Customer>,
    {
    pub fn new(op_query: OpQuery, customer_query: CustomerQuery) -> Self {
        SearchClinicService { op_query, customer_query } 
    }
    }

impl<OpQuery, CustomerQuery> QueryHandler<SearchClinicParamObj, Clinic>
    for SearchClinicService<OpQuery, CustomerQuery>
where
    OpQuery: QueryHandler<SearchOpParamObj, Op>,
    CustomerQuery: QueryHandler<SearchCustomerParamObj, Customer>,
{
    fn handle(&self, param_object: SearchClinicParamObj) -> Result<Clinic, CustomError> {
        let found_op = self
            .op_query.handle(param_object.op_param_obj)?;

        let found_customer = self
            .customer_query.handle(param_object.customer_param_obj)?;

        Ok(Clinic::new(found_op, found_customer))
    }
}

use clinicquery::{SearchClinicService, SearchClinicParamObj};
use opquery::{SearchOpService, Op, SearchOpParamObj};
use customerquery::{SearchCustomerService, Customer, SearchCustomerParamObj};
use queryhandler::{QueryHandler, CustomError};

pub mod clinicquery;
pub mod customerquery;
pub mod opquery;
pub mod queryhandler;

fn main() -> Result<(), CustomError>{
    let ops = vec![Op::new(2, "dickmed".to_string()), Op::new(1, "hplehnen".to_string())];
    let opservice = SearchOpService{ops};
    let customers = vec![Customer::new(2, "d".to_string(), "m".to_string()),Customer::new(1, "h".to_string(), "p".to_string())];
    let customerservice = SearchCustomerService{customers};
    let clinicserice = SearchClinicService::new(opservice, customerservice);
    let clinic_param = SearchClinicParamObj::new(1, 1);

    // let op_id = SearchOpParamObj::new(1);
    // let op = opservice.handle(op_id)?;
    // let customer_id = SearchCustomerParamObj::new(1);
    // let customer = customerservice.handle(customer_id)?;

    let clinic = clinicserice.handle(clinic_param)?;
    println!("Found clinic is: {:?}", clinic);
    Ok(())
}


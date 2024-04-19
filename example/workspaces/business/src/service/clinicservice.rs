use crate::{
    error::customerror::CustomError, models::clinic::Clinic,
    traits::command_handler::CommandHandler, traits::repository::Repository,
};

pub struct AddClinic {
    pub clinic_name: String,
}

pub struct AddClinicService<Rep: Repository<Clinic>> {
    pub repository: Rep,
}

impl<Rep> AddClinicService<Rep>
where
    Rep: Repository<Clinic>,
{
    pub fn add_clinic(&self, _add_clinic: AddClinic) {
    }
    pub fn new(repository: Rep) -> Self {
        AddClinicService { repository }
    }
}

impl<Rep> CommandHandler<AddClinic, i32> for AddClinicService<Rep>
where
    Rep: Repository<Clinic>,
{
    fn handle(&mut self, param_object: AddClinic) -> Result<i32, CustomError> {
        let clinic = Clinic {
            clinic_id: None,
            clinic_name: param_object.clinic_name,
        };
        let res = self.repository.postsomething(clinic);
        match res {
            Ok(id) => print!("\nThe id is {:?} \n", id),
            Err(_) => print!("\nThere was an error. \n"),
        }
        res
    }
}

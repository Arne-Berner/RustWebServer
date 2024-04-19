use business::service::clinicservice::AddClinic;
use business::traits::command_handler::CommandHandler;

pub struct Controller<Handler: CommandHandler<AddClinic, i32>> {
    add_clinic_service: Handler,
}


impl<Handler> Controller<Handler>
where Handler: CommandHandler<AddClinic, i32>{
    pub fn control_things(mut self) {
        let param_obj = AddClinic {
            clinic_name: "UKSH".to_string(),
        };
        //this would only trigger when a certain route is triggered.
        let res = self.add_clinic_service.handle(param_obj);
        match res{
            Ok(_) => println!("Handling Successfull!"),
            Err(_) => println!("here should be some error conversion.")
        }
    }
    pub fn new(add_clinic_service: Handler) -> Self{
        Controller{add_clinic_service}
    }
}

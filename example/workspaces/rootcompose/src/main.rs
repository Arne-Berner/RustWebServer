use business::service::clinicservice::*;
use repository::clinic_repo::ClinicRepo;
// what a name
use controller::controller::Controller;
use testlibrary::test;

fn main() {
    let repo = ClinicRepo::new(None, "Nothingness".to_string());
    let add_clinic_service = AddClinicService::new(repo);
    let controller = Controller::new(add_clinic_service);
    controller.control_things();
    test::test();

}

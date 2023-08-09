use business::{
    error::{
        customerror::CustomError,
        statuscode::StatusCode,
    },
    models::clinic::Clinic,
    traits::repository::Repository,
};

pub struct ClinicRepo {
    clinic: Clinic,
}

impl Repository<Clinic> for ClinicRepo {
    fn getsomething(&self) -> Result<Clinic, CustomError> {
        Ok(Clinic {
            clinic_id: self.clinic.clinic_id,
            clinic_name: self.clinic.clinic_name.clone(),
        })
    }

    fn deletesomething(&mut self, id: i32) -> Result<(), CustomError> {
        // wichtig: die Klammern um if statements sind sehr rust untypisch
        if self.clinic.clinic_id == Some(id) {
            let _ = self.clinic.clinic_id == Some(-1);
            let _ = self.clinic.clinic_name == format!("");
        }
        return Ok(());
    }

    fn postsomething(&mut self, mut clinic: Clinic) -> Result<i32, CustomError> {
        // sollte natürlich eigentlich eine Clinic posten, aber ersetzt sie hier einfachheitshalber
        clinic.clinic_id = Some(1);
        self.clinic = clinic;
        println!("Added {:?}", self.clinic);
        Ok(self.clinic.clinic_id.unwrap())
    }
}

impl ClinicRepo {
    /// Creates a new [`ClinicRepo`].
    pub fn new(clinic_id: Option<i32>, clinic_name: String) -> Self {
        ClinicRepo {
            clinic: Clinic {
                clinic_id,
                clinic_name,
            },
        }
    }
}

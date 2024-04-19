#[derive(Debug)]
pub struct Clinic{
    // sollte nicht pub sein, ist nur kürzer für den Code hier
    pub clinic_id: Option<i32>,
    pub clinic_name: String
}
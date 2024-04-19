#[derive(Debug)]
pub enum StatusCode {
    Okay,
    InternalServerCode
}

#[derive(Debug)]
pub struct Okay{
    pub number: i32,
    pub status_message: String
}

impl Okay{
    pub fn new() -> Self{
        Okay{number: 200,
        status_message: format!("Everything is fine")}
    }
}

#[derive(Debug)]
pub struct InternalServerCode{
    pub number: i32,
    pub status_message: String
}

impl InternalServerCode{
    pub fn new() -> Self{
        InternalServerCode{number: 500,
        status_message: format!("A problem in the backend occured")}
    }
}
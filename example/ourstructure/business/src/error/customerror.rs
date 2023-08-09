use crate::error::statuscode::InternalServerCode;
use crate::error::statuscode::StatusCode;
#[derive(Debug)]
pub enum CustomError {
    InternalError(InternalServerCode),
    ExternalError,
}

struct InternalError {
    status_code: StatusCode,
}

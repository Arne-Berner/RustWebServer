use crate::error::customerror::CustomError;

pub trait CommandHandler<ParamObject, Res> {
    fn handle(&mut self, param_obj: ParamObject) -> Result<Res, CustomError>;
}
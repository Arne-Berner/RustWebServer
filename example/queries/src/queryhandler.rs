pub trait QueryHandler<ParamObject, Entity> {
    fn handle(&self, param_obj: ParamObject) -> Result<Entity, CustomError>;
}

#[derive(Debug)]
pub enum CustomError {
    Internal,
    External,
}
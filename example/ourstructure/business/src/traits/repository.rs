use crate::error::customerror::CustomError;

pub trait Repository<Entity> {
    fn getsomething(&self) -> Result<Entity, CustomError>;
    fn postsomething(&mut self, entity: Entity) -> Result<i32, CustomError>;
    fn deletesomething(&mut self, id: i32) -> Result<(), CustomError>;
}

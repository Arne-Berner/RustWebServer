use crate::error::customerror::CustomError;

pub trait Repository<Entity> {
    async fn getsomething(&self) -> Result<Entity, CustomError>;
    async fn postsomething(&mut self, entity: Entity) -> Result<i32, CustomError>;
    async fn deletesomething(&mut self, id: i32) -> Result<(), CustomError>;
}

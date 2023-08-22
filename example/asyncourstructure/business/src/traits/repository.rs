use crate::error::customerror::CustomError;

pub trait Repository {
    type Entity;
    async fn getsomething(&self) -> Result<Self::Entity, CustomError>;
    async fn postsomething(&mut self, entity: Self::Entity) -> Result<i32, CustomError>;
    async fn deletesomething(&mut self, id: i32) -> Result<(), CustomError>;
}

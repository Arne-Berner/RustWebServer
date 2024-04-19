use compose::get_do_something;

fn main() {
    let do_something = get_do_something();
    do_something.run();
}

pub mod business {
    pub trait Repository {
        type Entity;

        fn getsomething(&self) -> Result<Self::Entity, CustomError>;
        fn postsomething(&mut self, entity: Self::Entity) -> Result<i32, CustomError>;
        fn deletesomething(&mut self, id: i32) -> Result<(), CustomError>;
    }

    pub struct Service<Rep> {
        pub repository: Rep,
    }

    impl<Rep> Service<Rep>
    where
        Rep: Repository + std::fmt::Debug,
    {
        pub fn run(&self)
        where
            <Rep as Repository>::Entity: std::fmt::Debug,
        {
            let entity = &self.repository.getsomething();
            match entity {
                Ok(n) => print!("\nThe name is {:?} \n", n),
                Err(_) => print!("\nSomething bad happened!"),
            }
        }
        pub fn new(repository: Rep) -> Self {
            Service { repository }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct ExampleEntity{
        pub name: &'static str,
    }

    pub struct CustomError;
}

pub mod data {
    use crate::business::{ExampleEntity, Repository};


    #[derive(Debug)]
    pub struct RealRepo{
        entity: ExampleEntity,

    }

    impl RealRepo {
        pub fn new() -> Self {
            RealRepo {
                entity: ExampleEntity { name: "Bob" },
            }
        }
    }

    impl Repository for RealRepo {
        type Entity = ExampleEntity;

        fn getsomething(&self) -> Result<Self::Entity, crate::business::CustomError> {
            Result::Ok(self.entity)
        }

        fn postsomething(
            &mut self,
            entity: Self::Entity,
        ) -> Result<i32, crate::business::CustomError> {
            self.entity = entity;
            // this could be the ID of the entity
            Result::Ok(1)
        }

        fn deletesomething(&mut self, _id: i32) -> Result<(), crate::business::CustomError> {
            todo!()
        }
    }

    // This showcases the typesafety.
    // Since this does not implement Debug, it cannot be used in Service.
    pub struct NonDebugRepo{
        entity: ExampleEntity,
    }

    impl NonDebugRepo{
        pub fn new() -> Self {
            NonDebugRepo { entity: ExampleEntity { name: "None" } }
        }
    }

    impl Repository for NonDebugRepo{
        type Entity = ExampleEntity;

        fn getsomething(&self) -> Result<Self::Entity, crate::business::CustomError> {
            todo!()
        }

        fn postsomething(&mut self, entity: Self::Entity) -> Result<i32, crate::business::CustomError> {
            todo!()
        }

        fn deletesomething(&mut self, id: i32) -> Result<(), crate::business::CustomError> {
            todo!()
        }
    }
}

pub mod compose {
    use crate::{business::Service, data::{RealRepo, NonDebugRepo}};

    pub fn get_do_something() -> Service<RealRepo> {
        Service::new(RealRepo::new())
    }
    // This will not work, since nonDebug does not implement Debug
    // pub fn get_do_something() -> Service<NonDebugRepo> {
    //     Service::new(NonDebugRepo::new())
    // }
}
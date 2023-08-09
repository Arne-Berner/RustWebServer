use compose::{get_do_something};

fn main() {
    let do_something = get_do_something();
    do_something.run();
}


pub mod business {
    pub struct Service<Rep: Repository> {
        pub repository: Rep,
    }

    pub trait Repository {
        fn getsomething(&self) -> &str;
    }

    impl<Rep> Service<Rep>
    where
        Rep: Repository,
    {
        pub fn run(&self) {
            let name = &self.repository.getsomething();
            print!("\nThe name is {} \n", name);
        }
        pub fn new(repository: Rep) -> Self {
            Service { repository }
        }
    }
}

pub mod data {
    use crate::business::Repository;

    pub struct RealRepo<'a> {
        firstname: &'a str,
    }

    impl Repository for RealRepo<'_> {
        fn getsomething(&self) -> &str {
            &self.firstname
        }
    }

    impl RealRepo<'_> {
        pub fn new() -> Self {
            RealRepo { firstname: "John" }
        }
    }
}

pub mod compose {
    use crate::{
        business::{Service},
        data::RealRepo,
    };


        pub fn get_do_something() -> Service<RealRepo<'static>> {
            Service::new(RealRepo::new())
        }
}

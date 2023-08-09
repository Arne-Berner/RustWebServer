use business::{
    error::customerror::CustomError, models::test::Test,
    traits::command_handler::CommandHandler, traits::repository::Repository,
};

pub struct AddTest {
    pub test_name: String,
}

pub struct AddTestService<Rep: Repository<Test>> {
    pub repository: Rep,
}

impl<Rep> AddTestService<Rep>
where
    Rep: Repository<Test>,
{
    pub fn add_test(&self, add_test: AddTest) {
    }
    pub fn new(repository: Rep) -> Self {
        AddTestService { repository }
    }
}

impl<Rep> CommandHandler<AddTest, i32> for AddTestService<Rep>
where
    Rep: Repository<Test>,
{
    async fn handle(&mut self, param_object: AddTest) -> Result<i32, CustomError> {
        let test = Test {
            test_id: None,
            test_name: param_object.test_name,
        };
        let res = self.repository.postsomething(test).await;
        match res {
            Ok(id) => print!("\nThe id is {:?} \n", id),
            Err(_) => print!("\nThere was an error. \n"),
        }
        res
    }
}

#![feature(async_fn_in_trait)]


/// ```
/// use async_example::Service;
/// use async_example::CustomError;
/// use async_example::AsyncCommandHandler;
/// use async_example::secondprint;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), CustomError>{
///    let mut service = Service{};
///    let (_, res) = tokio::join!(secondprint(), service.handle(()));
///    res
/// }
/// ```

use tokio::time::sleep_until;
///```
/// use async_example::Service;
/// use async_example::CustomError;
/// use async_example::AsyncCommandHandler;
/// #[tokio::main]
/// async fn main() -> Result<(), CustomError>{
///    let mut service = Service{};
///    service.handle(()).await
/// }
///```
pub trait AsyncCommandHandler{
    type ParamObject;
    type ReturnType;
    async fn handle(&mut self, param_obj: Self::ParamObject)
        -> Result<Self::ReturnType, CustomError>;
}

#[derive(Debug)]
pub struct CustomError{}

#[derive(Debug, Clone)]
pub struct Service{}

impl AsyncCommandHandler for Service{
    type ParamObject = ();
    type ReturnType = ();
    async fn handle(&mut self, param_obj: Self::ParamObject)
        -> Result<Self::ReturnType, CustomError> {
            // each await will poll if its ready and yield if not
            // that way other functions that are above this await in the callstack
            // can try fullfilling their future.
            asyncprint().await;
            asyncprint().await;
            asyncprint().await;
            Ok(())
    }
}

async fn asyncprint(){
    println!("This is async!");
    sleep_until(tokio::time::Instant::now()+ tokio::time::Duration::from_millis(1000)).await;
}

pub async fn secondprint(){
    second().await;
    second().await;
    second().await;
}

async fn second(){
    sleep_until(tokio::time::Instant::now()+ tokio::time::Duration::from_millis(900)).await;
    println!("Secondasync");
}

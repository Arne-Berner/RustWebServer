
use crate::error::customerror::CustomError;

pub trait CommandHandler<ParamObject, Res> {
   async fn handle(&mut self, param_obj: ParamObject) -> Result<Res, CustomError>;
}

// this could be the solution to the whole "async" problem
//trait Handler {
//    type Future: Future<Output = Result<HttpResponse, Error>>;
//
//    fn call(&mut self, request: HttpRequest) -> Self::Future;
//}

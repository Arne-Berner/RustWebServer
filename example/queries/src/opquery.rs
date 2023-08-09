use crate::queryhandler::*;

#[derive(Debug, Clone)]
pub struct Op {
    id: u32,
    name: String,
}

impl Op {
    pub fn new(id: u32, name: String) -> Self {
        Op { id, name}
    }
}

#[derive(Debug)]
pub struct SearchOpParamObj {
    id: u32,
}

impl SearchOpParamObj {
    pub fn new(id: u32) -> Self {
        SearchOpParamObj { id}
    }
}

pub struct SearchOpService {
    pub ops: Vec<Op>
}

impl QueryHandler<SearchOpParamObj, Op> for SearchOpService {
    fn handle(&self, param_object: SearchOpParamObj) -> Result<Op, CustomError> {
        let found_op = self.ops.iter().find(|op| op.id == param_object.id);

        match found_op {
            Some(op) =>Ok(op.clone()),
            None => Err(CustomError::Internal),
        }
    }
}
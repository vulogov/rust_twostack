use crate::stack::Stack;
use rust_dynamic::value::Value;
use nanoid::nanoid;

#[derive(Clone)]
pub enum StackOp {
    None,
    TakeOne,
    TakeTwo,
    TakeAll,
}

#[derive(Clone)]
pub struct TS{
    pub id:             String,
    pub stack:          Stack<Stack<Value>>
}

impl TS {
    fn init() -> Self {
        Self {
            id:     nanoid!(),
            stack:  Stack::new(),
        }
    }
    pub fn new() -> Self {
        let mut res = TS::init();
        res.add_stack();
        res
    }
}

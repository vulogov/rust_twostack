use crate::stack::Stack;
use rust_dynamic::value::Value;
use nanoid::nanoid;

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

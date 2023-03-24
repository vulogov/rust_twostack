use crate::ts::TS;
use crate::stack::Stack;
use rust_dynamic::value::Value;

impl TS {
    pub fn current(&mut self) -> Option<&mut Stack<Value>> {
        match self.ensure().stack.peek() {
            Some(curr) => Some(curr),
            None => {
                return None
            }
        }
    }
    pub fn pull(&mut self) -> Option<Value> {
        match self.stack.peek() {
            Some(curr) => curr.pull(),
            None => {
                return None
            }
        }
    }
}

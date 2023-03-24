use crate::ts::TS;
use crate::stack::Stack;
use rust_dynamic::value::Value;

impl TS {
    pub fn push(&mut self, value: Stack<Value>) -> &mut TS {
        self.stack.push(value);
        self
    }
    pub fn add_stack(&mut self) -> &mut TS {
        self.stack.push(Stack::new());
        self
    }
}

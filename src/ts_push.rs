use crate::ts::TS;
use crate::stack::Stack;
use rust_dynamic::value::Value;

impl TS {
    pub fn push(&mut self, value: Value) -> &mut TS {
        match self.stack.peek() {
            Some(curr) => {
                curr.push(value);
                return self;
            }
            None => {
                self.add_stack();
                return self.push(value);
            }
        }
    }
    pub fn add_stack(&mut self) -> &mut TS {
        self.stack.push(Stack::new());
        self
    }
    pub fn add_named_stack(&mut self, name: String) -> &mut TS {
        self.stack.push(Stack::init(name));
        self
    }
}

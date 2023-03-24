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
    pub fn swap(&mut self) -> &mut TS {
        if self.stack_len() >= 2 {
            match self.pull() {
                Some(x) => {
                    match self.pull() {
                        Some(y) => {
                            self.push(x);
                            self.push(y);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
        self
    }
    pub fn dup(&mut self) -> &mut TS {
        if self.stack_len() >= 1 {
            match self.pull() {
                Some(x) => {
                    match x.dup() {
                        Ok(y) => {
                            self.push(x).push(y);
                        }
                        Err(_) => {}
                    }
                }
                None => {}
            }
        }
        self
    }
}

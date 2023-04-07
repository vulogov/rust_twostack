use crate::ts::{TS, StackOp};
use rust_dynamic::value::Value;

impl TS {
    pub fn apply(&mut self, value: Value, op: StackOp) -> &mut TS {
        match op {
            StackOp::None => {
                self.push(value);
            }
            StackOp::TakeOne => {
                if self.stack_len() >= 1 {
                    match self.pull() {
                        Some(obj) => {
                            let mut v = value.dup().unwrap();
                            v = v.attr_merge(vec![obj]);
                            self.push(v);
                        }
                        _ => {},
                    }
                }
            }
            StackOp::TakeAll => {
                let mut a: Vec<Value> = Vec::new();
                let mut v = value.dup().unwrap();
                while self.stack_len() > 0 {
                    match self.pull() {
                        Some(obj) => {
                            a.push(obj);
                        }
                        _ => break,
                    }
                }
                v = v.attr_merge(a);
                self.push(v);
            }
        }
        self
    }
}

use crate::ts::{TS, StackOp};
use rust_dynamic::value::Value;

impl TS {
    pub fn value(&mut self, value: Value, op: StackOp) -> Result<Value, Box<dyn std::error::Error>> {
        match op {
            StackOp::None => {
                return Result::Ok(value);
            }
            StackOp::TakeOne => {
                if self.stack_len() >= 1 {
                    match self.pull() {
                        Some(obj) => {
                            match value.dup() {
                                Ok(mut v) => {
                                    v = v.attr_merge(vec![obj]);
                                    return Result::Ok(v);
                                }
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }
                        _ => {},
                    }
                }
            }
            StackOp::TakeTwo => {
                if self.stack_len() >= 2 {
                    let mut nv: Vec<Value> = Vec::new();
                    for _ in 0..2 {
                        match self.pull() {
                            Some(obj) => nv.push(obj),
                            _ => break,
                        }
                    }
                    match value.dup() {
                        Ok(mut v) => {
                            v = v.attr_merge(nv);
                            return Result::Ok(v);
                        }
                        Err(err) => {
                            return Err(err);
                        }
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
                return Result::Ok(v);
            }
        }
        Result::Ok(value)
    }
}

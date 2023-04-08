use crate::ts::{TS, StackOp};
use rust_dynamic::value::Value;
use rust_dynamic::ctx::{Context, CtxAppFn};

impl TS {
    pub fn eval(&mut self, ctx: &dyn Context, n: &str, f: CtxAppFn, value: Value, op: StackOp) -> Result<&mut TS, Box<dyn std::error::Error>> {
        match self.value(value, op) {
            Ok(val) => {
                match (f)(ctx, n, val) {
                    Some(res) => {
                        self.push(res);
                    }
                    None => {},
                }
                return Result::Ok(self);
            }
            Err(err) => return Err(err),
        }
    }
}

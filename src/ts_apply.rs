use crate::ts::{TS, StackOp};
use rust_dynamic::value::Value;

impl TS {
    pub fn apply(&mut self, value: Value, op: StackOp) -> Result<&mut TS, Box<dyn std::error::Error>> {
        match self.value(value, op) {
            Ok(val) => {
                self.push(val);
                return Result::Ok(self);
            }
            Err(err) => return Err(err),
        }
    }
}

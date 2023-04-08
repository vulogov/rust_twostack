#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_twostack::ts::{TS, StackOp};
    use rust_dynamic::value::Value;
    use rust_dynamic::ctx::Context;
    use rust_dynamic::ctx::NullContext;

    #[test]
    fn test_ts_eval_val() {
        let mut ts = TS::new();
        let ctx = NullContext::new();
        let fun = ctx.resolve("test").expect("function expected").f;
        let _ = ts.eval(&ctx, "test", fun, Value::from_int(42), StackOp::None).unwrap();
        assert_eq!(ts.stack_len(), 0);
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_twostack::ts::{TS, StackOp};
    use rust_dynamic::value::Value;

    #[test]
    fn test_ts_apply_check_len() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
            .apply(
                Value::from_float(42.0)
                       .attr_add(Value::from_int(0)),
                StackOp::None
            );
        assert_eq!(ts.stack_len(), 1);
    }
    #[test]
    fn test_ts_apply_check_attr_len() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
            .apply(
                Value::from_float(42.0)
                       .attr_add(Value::from_int(0)),
                StackOp::None
            );
        let v = ts.pull().unwrap();
        assert_eq!(v.attr_len(), 1);
    }
    #[test]
    fn test_ts_apply_check_attr_val() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
            .apply(
                Value::from_float(42.0)
                       .attr_add(Value::from_int(0)),
                StackOp::None
            );
        let v = ts.pull().unwrap();
        assert_eq!(v.attr[0].cast_int().unwrap(), 0);
    }
    #[test]
    fn test_ts_apply_check_attr_takeone_len() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
            .push(Value::from_int(42))
            .apply(
                Value::from_float(42.0)
                       .attr_add(Value::from_int(0)),
                StackOp::TakeOne
            );
        let v = ts.pull().unwrap();
        assert_eq!(v.attr_len(), 2);
    }
    #[test]
    fn test_ts_apply_check_attr_takeone_val() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
            .push(Value::from_int(42))
            .apply(
                Value::from_float(42.0)
                       .attr_add(Value::from_int(0)),
                StackOp::TakeOne
            );
        let v = ts.pull().unwrap();
        assert_eq!(v.attr[0].cast_int().unwrap(), 42);
    }
}

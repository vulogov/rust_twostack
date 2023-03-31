#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_twostack::ts::TS;
    use rust_dynamic::value::Value;

    #[test]
    fn test_ts_new() {
        let ts = TS::new();
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn test_ts_clear() {
        let mut ts = TS::new();
        ts.add_stack()
          .add_stack()
          .clear();
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn test_ts_ensure() {
        let mut ts = TS::new();
        ts.ensure();
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn test_ts_add_named_stack() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string());
        let s = ts.current().expect("No value been pulled");
        assert_eq!(s.stack_id(), "B");
    }

    #[test]
    fn test_ts_position_named_stack() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string())
          .position("A".to_string());
        let s = ts.current().expect("No value been pulled");
        assert_eq!(s.stack_id(), "A");
    }

    #[test]
    fn test_ts_push() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap());
        assert_eq!(ts.stack_len(), 2);
    }

    #[test]
    fn test_ts_pull() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap());
        let val = ts.pull().expect("No value been pulled");
        assert_eq!(val.cast_float().unwrap(), 42.0);
    }

    #[test]
    fn test_ts_direct_stack_op() {
        let mut ts = TS::new();
        let mut s = ts.current().expect("No value been pulled");
        s = s.push(Value::from(42.0).unwrap());
        let val = s.pull().expect("No value been pulled");
        assert_eq!(val.cast_float().unwrap(), 42.0);
    }

    #[test]
    fn test_ts_swap() {
        let mut ts = TS::new();
        ts.push(Value::from(42.0).unwrap())
          .push(Value::from(41.0).unwrap())
          .swap();
        let val = ts.pull().expect("No value been pulled");
        assert_eq!(val.cast_float().unwrap(), 42.0);
    }

    #[test]
    fn test_ts_dup() {
        let mut ts = TS::new();
        ts.push(Value::from(42.0).unwrap())
          .dup();
        let val = ts.pull().expect("No value been pulled");
        assert_eq!(val.cast_float().unwrap(), 42.0);
    }

    #[test]
    fn test_ts_dup_check_len() {
        let mut ts = TS::new();
        ts.push(Value::from(42.0).unwrap())
          .dup();
        assert_eq!(ts.stack_len(), 2);
    }
    #[test]
    fn test_ts_drop_named_stack() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string())
          .drop();
        let s = ts.current().expect("No value been pulled");
        assert_eq!(s.stack_id(), "A");
    }
    #[test]
    fn test_ts_return_to() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
            .add_named_stack("B".to_string())
            .push(Value::from(42.0).unwrap())
            .return_to();
        let v = ts.pull().expect("No value been pulled");
        assert_eq!(v.cast_float().unwrap(), 42.0 as f64);
    }

}

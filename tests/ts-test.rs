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
    fn test_ts_push() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap());
        assert_eq!(ts.stack_len(), 2);
    }

}

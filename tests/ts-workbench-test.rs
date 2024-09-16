#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_twostack::ts::TS;
    use rust_dynamic::value::Value;

    #[test]
    fn test_workbench_return_to1() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .return_to();
        assert_eq!(ts.len(), 1);
    }
    #[test]
    fn test_workbench_return_to2() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .return_to();
        assert_eq!(ts.workbench_len(), 1);
    }
    #[test]
    fn test_workbench_return_to3() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .return_to();
        ts.return_from();
        assert_eq!(ts.len(), 2);
    }
}

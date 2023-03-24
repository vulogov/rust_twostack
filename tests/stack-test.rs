#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_twostack::stack::Stack;

    #[test]
    fn test_stack_new() {
        let s: Stack<Value> = Stack::new();
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_stack_push() {
        let mut s: Stack<Value> = Stack::new();
        s.push(Value::from(42.0).unwrap())
         .push(Value::from(41.0).unwrap());
        assert_eq!(s.len(), 2);
    }
}

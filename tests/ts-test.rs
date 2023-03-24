#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_twostack::ts::TS;

    #[test]
    fn test_ts_new() {
        let ts = TS::new();
        assert_eq!(ts.len(), 1);
    }

}

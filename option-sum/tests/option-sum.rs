use option_sum::*;

#[test]
fn test_sum() {
    assert_eq!(None, sum(&[u32::MAX, 10u32]));
    assert_eq!(Some(100), sum(&[10;10]));
}

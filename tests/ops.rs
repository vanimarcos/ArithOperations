use arith_operations::operations::{add, sub, multiply, divide};


#[test]
fn test_add() {
    assert_eq!(add(2, 1), 3);
}

#[test]
fn test_sub() {
    assert_eq!(sub(3, 2), 1);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(2, 1), 2);
}

#[test]
fn test_division() {
    assert_eq!(divide(2, 2), 1 as f64);
}

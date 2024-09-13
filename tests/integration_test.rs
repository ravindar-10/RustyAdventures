use rustyadventures::{add, divide, multiply, subtract};

#[test]
fn integration_test_add() {
    assert_eq!(add(10, 20), 30);
}

#[test]
fn integration_test_subtract() {
    assert_eq!(subtract(20, 10), 10);
}

#[test]
fn integration_test_multiply() {
    assert_eq!(multiply(4, 5), 20);
}

#[test]
fn integration_test_divide() {
    assert_eq!(divide(20, 4), Some(5));
    assert_eq!(divide(20, 0), None);
}

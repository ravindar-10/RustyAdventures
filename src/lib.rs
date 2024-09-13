/// A simple arithmetic library.

/// Adds two integers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second integer from the first.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two integers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides the first integer by the second. Returns `None` if dividing by zero.
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 6);
        assert_eq!(add(-2, -3), -5);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(-2, -3), 1);
        assert_eq!(subtract(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(-2, -3), 6);
        assert_eq!(multiply(0, 5), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6, 3), Some(2));
        assert_eq!(divide(-6, -3), Some(2));
        assert_eq!(divide(5, 2), Some(2));
        assert_eq!(divide(5, 0), None);
    }
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_positive_numbers() {
        assert_eq!(addition(2, 3), 5);
        assert_eq!(addition(10, 15), 25);
    }

    #[test]
    fn test_addition_negative_numbers() {
        assert_eq!(addition(-2, -3), -5);
        assert_eq!(addition(-10, 5), -5);
    }

    #[test]
    fn test_addition_zero() {
        assert_eq!(addition(0, 5), 5);
        assert_eq!(addition(5, 0), 5);
        assert_eq!(addition(0, 0), 0);
    }

    #[test]
    fn test_addition_large_numbers() {
        assert_eq!(addition(1000000, 2000000), 3000000);
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test_addition_positive_numbers() {
        assert_eq!(addition(2, 3), 5);
        assert_eq!(addition(10, 15), 25);
    }

    #[test]
    fn test_addition_negative_numbers() {
        assert_eq!(addition(-2, -3), -5);
        assert_eq!(addition(-10, 5), -5);
    }

    #[test]
    fn test_addition_zero() {
        assert_eq!(addition(0, 5), 5);
        assert_eq!(addition(5, 0), 5);
        assert_eq!(addition(0, 0), 0);
    }

    #[test]
    fn test_addition_large_numbers() {
        assert_eq!(addition(1000000, 2000000), 3000000);
    }
}

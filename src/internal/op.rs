pub fn add(left: f64, right: f64) -> Option<f64> {
    Some(left + right)
}

pub fn subtract(left: f64, right: f64) -> Option<f64> {
    Some(left - right)
}

pub fn multiply(left: f64, right: f64) -> Option<f64> {
    Some(left * right)
}

pub fn divide(left: f64, right: f64) -> Option<f64> {
    if left == 0.0 || right == 0.0 {
        None
    } else {
        Some(left / right)
    }
}

pub fn exponent(left: f64, right: f64) -> Option<f64> {
    let mut res = 1.0;

    for _ in 0..right as i32 {
        res *= left;
    }

    Some(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(9.0, 10.0);

        assert_eq!(result, Some(19.0));
    }

    #[test]
    fn subtract_works() {
        let positive = subtract(19.0, 3.0);
        let negative = subtract(3.0, 4.0);

        assert_eq!(positive, Some(16.0));
        assert_eq!(negative, Some(-1.0));
    }

    #[test]
    fn multiply_works() {
        let twelve = multiply(3.0, 4.0);
        let zero = multiply(39.0, 0.0);

        assert_eq!(twelve, Some(12.0));
        assert_eq!(zero, Some(0.0));
    }

    #[test]
    fn divide_works() {
        let none = divide(3.0, 4.0);
        let five = divide(26.0, 5.0);

        assert_eq!(none, None);
        assert_eq!(five, Some(5.2));
    }

    #[test]
    fn exponent_works() {
        let result = exponent(3.0, 4.0);

        assert_eq!(result, Some(81.0));
    }
}

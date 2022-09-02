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

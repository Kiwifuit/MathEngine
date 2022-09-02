use std::str::FromStr;

mod op;
mod parser;

#[derive(Debug, PartialEq, Eq)]
pub enum MathOperators {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

impl MathOperators {
    fn to_callback(&self) -> fn(f64, f64) -> Option<f64> {
        match self {
            Self::Add => op::add,
            Self::Subtract => op::subtract,
            Self::Multiply => op::multiply,
            Self::Divide => op::divide,
            Self::Exponent => op::exponent,
        }
    }
}

impl FromStr for MathOperators {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            "^" => Ok(Self::Exponent),
            op => Err(format!("Unknown Operator: {}", op)),
        }
    }
}

pub struct MathOperation {
    left: f64,
    right: f64,
    opcode: MathOperators,
}

impl MathOperation {
    fn new(left: f64, right: f64, opcode: MathOperators) -> Self {
        Self {
            left,
            right,
            opcode,
        }
    }

    pub fn perform(&self) -> f64 {
        let operation = self.opcode.to_callback();
        match operation(self.left, self.right) {
            Some(num) => num,
            None => 0.0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{MathOperation, MathOperators};

    #[test]
    fn math_op_translation() {
        let a = "+".parse::<MathOperators>();
        let s = "-".parse::<MathOperators>();
        let m = "*".parse::<MathOperators>();
        let d = "/".parse::<MathOperators>();
        let e = "^".parse::<MathOperators>();

        let f = "?".parse::<MathOperators>();

        assert_eq!(a.unwrap(), MathOperators::Add);
        assert_eq!(s.unwrap(), MathOperators::Subtract);
        assert_eq!(m.unwrap(), MathOperators::Multiply);
        assert_eq!(d.unwrap(), MathOperators::Divide);
        assert_eq!(e.unwrap(), MathOperators::Exponent);
        assert!(f.is_err());
    }

    #[test]
    fn math_op_perform() {
        let add = MathOperation::new(1.0, 2.0, MathOperators::Add);
        let sub = MathOperation::new(3.0, 2.0, MathOperators::Subtract);
        let mul = MathOperation::new(2.0, 2.0, MathOperators::Multiply);
        let div = MathOperation::new(10.0, 5.0, MathOperators::Divide);
        let exp = MathOperation::new(2.0, 2.0, MathOperators::Exponent);

        assert_eq!(add.left, 1.0);
        assert_eq!(add.right, 2.0);
        assert_eq!(add.opcode, MathOperators::Add);
        assert_eq!(add.perform(), 3.0);

        assert_eq!(sub.left, 3.0);
        assert_eq!(sub.right, 2.0);
        assert_eq!(sub.opcode, MathOperators::Subtract);
        assert_eq!(sub.perform(), 1.0);

        assert_eq!(mul.left, 2.0);
        assert_eq!(mul.right, 2.0);
        assert_eq!(mul.opcode, MathOperators::Multiply);
        assert_eq!(mul.perform(), 4.0);

        assert_eq!(div.left, 10.0);
        assert_eq!(div.right, 5.0);
        assert_eq!(div.opcode, MathOperators::Divide);
        assert_eq!(div.perform(), 2.0);

        assert_eq!(exp.left, 2.0);
        assert_eq!(exp.right, 2.0);
        assert_eq!(exp.opcode, MathOperators::Exponent);
        assert_eq!(exp.perform(), 4.0);
    }
}

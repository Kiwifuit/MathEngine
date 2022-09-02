use std::str::FromStr;

mod op;
mod parser;

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
    pub fn perform(&self) -> f64 {
        let operation = self.opcode.to_callback();
        match operation(self.left, self.right) {
            Some(num) => num,
            None => 0.0,
        }
    }
}

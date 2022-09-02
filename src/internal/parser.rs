use pomsky_macro::pomsky;
use regex::Regex;

use super::MathOperators;

const ENGINE_PARSER: &str = pomsky!(
:left([d w]*)
' '*
:opcode(['+' '-' '/' '^' '*'])
' '*
:right([d w]*)
);

pub fn parse(raw: &str) -> Result<(f64, f64, MathOperators), String> {
    let parser = match Regex::new(&ENGINE_PARSER) {
        Ok(r) => r,
        Err(err) => return Err(err.to_string()),
    };
    let caps = parser.captures(&raw);

    if let Some(cap) = caps {
        // Here we are guaranteed to have values, so
        // its alright to unwrap everything

        let left = cap.name("left").unwrap().as_str();
        let opcode = cap.name("opcode").unwrap().as_str();
        let right = cap.name("right").unwrap().as_str();

        return Ok((
            left.parse::<f64>().unwrap(),
            right.parse::<f64>().unwrap(),
            opcode.parse::<MathOperators>().unwrap(),
        ));
    } else {
        return Err(format!("Unable to parse anything from: {}", raw));
    }
}

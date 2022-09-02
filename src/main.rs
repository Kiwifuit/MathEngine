use std::io::{stdin, stdout, Write};
use std::str::FromStr;

mod internal;

pub fn prompt<R>(message: &str, newline: bool) -> Option<R>
where
    R: FromStr,
    <R as FromStr>::Err: std::fmt::Display,
{
    loop {
        let mut raw = String::new();

        if newline {
            println!("{}", message);
        } else {
            print!("{}", message);
            stdout().flush().unwrap();
        }

        stdin()
            .read_line(&mut raw)
            .ok()
            .expect("Unble to get user input");

        match raw.trim().parse::<R>() {
            Ok(parsed) => return Some(parsed),
            Err(err) => println!("An Error Occured: {}", err),
        };
    }
}

fn main() {
    println!("Hello, world!");
}

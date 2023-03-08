use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MyError {
    code: i32,
    message: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]{}", self.code, self.message)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.message
    }
}

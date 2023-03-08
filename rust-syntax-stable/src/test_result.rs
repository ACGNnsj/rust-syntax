use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct MyError {
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

#[test]
fn test_result()->Result<(), MyError> {
    let result: Result<(), MyError> = Err(MyError {
        code: 1,
        message: "error".to_string(),
    });
    assert!(result.is_err());
    // assert_eq!(result.unwrap_err().description(), "error");
    result?;
    /*match result {
        Ok(v) => v,
        Err(e) => return Err(e.into())
    }*/
    Ok(())
}



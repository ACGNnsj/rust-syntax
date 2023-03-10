use std::fmt::Debug;
use public_struct::error::MyError;
use public_struct::show::Show;
use crate::test_orphan_rule::r#struct::AnyStruct;
use crate::test_orphan_rule::r#trait::{AnyTrait, LocalTrait};

impl AnyTrait for AnyStruct {
    fn any_trait(&self) -> usize {
        0
    }
}

#[test]
fn test() {
    println!("{:?}", AnyStruct::new().any_trait());
}


/*impl Show for MyError {
    fn show(&self) -> String {
        format!("[{}]{}", self.code, self.message)
    }
}

#[test]
fn test_(){
    println!("{:?}", MyError::new(1, "test".to_string()).show());
}*/

/*impl Show<dyn LocalTrait> for MyError {
    fn show(&self) -> String {
        format!("[{}]{}", self.code, self.message)
    }
}*/

/*impl<T: LocalTrait> Show for MyError {
    type ShowType = T;

    fn show(&self) -> String {
        format!("[{}]{}", self.code, self.message)
    }
}*/

impl Show<Box<dyn LocalTrait>> for MyError {
    fn show(&self) -> String {
        format!("[{}]{}", self.code, self.message)
    }
}

#[test]
fn test_() {
    println!("{:?}", MyError::new(1, "test".to_string()).show());
}
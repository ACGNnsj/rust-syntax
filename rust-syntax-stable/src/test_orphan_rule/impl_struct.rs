use crate::test_orphan_rule::r#struct::AnyStruct;
use crate::test_orphan_rule::r#trait::AnyTrait;

impl AnyStruct {
    pub fn new() -> AnyStruct {
        AnyStruct {}
    }
}

#[test]
fn test(){
    println!("{:?}", AnyStruct::new());
}


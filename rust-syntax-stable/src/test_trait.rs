use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Display;
use std::panic::{RefUnwindSafe, UnwindSafe};

pub struct Cat {
    name: &'static str,
}


/*impl Clone for Cat {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Copy for Cat {}*/
// impl Sized for Cat {}

pub static mut CAT: Cat = Cat { name: "cat" };

fn test_sized(sized: impl Sized + RefUnwindSafe + Send + Sync + Unpin + UnwindSafe + Any + Borrow<Cat> + BorrowMut<Cat> + From<Cat> + Into<Cat> + TryFrom<Cat> + TryInto<Cat>) {
    println!("sized");
    pub static mut ANOTHER_CAT: Cat = Cat { name: "cat" };
}

#[test]
fn test_trait() {
    let cat = Cat { name: "cat" };
    test_sized(cat);
}
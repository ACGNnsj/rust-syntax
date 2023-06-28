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

fn foo<F:Foo>(foo:&F){
    println!("F");
}
trait Foo {
    fn foo(&self);

    // default implementation
    fn bar(&self) { println!("We called bar."); }
}

struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
    fn bar(&self) {
        println!("bar");
    }
}

struct Bar;

impl Foo for Bar {
    fn foo(&self) { println!("foo"); }
}


#[test]
fn test_trait2() {
    let baz = Baz;
    baz.foo();
    baz.bar();
    let bar = Bar;
    bar.foo();
    bar.bar();
}
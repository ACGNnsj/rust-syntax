// use crate::test_ufcs;

use std::fmt::{Debug, Formatter};

trait FooPrinter {
    fn print(&self) {
        println!("hello");
    }
}

impl FooPrinter for () {
    fn print(&self) {
        println!("{:?}", self)
    }
}

impl FooPrinter for (i32, u16) {
    fn print(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct Foo;

impl FooPrinter for Foo {
    fn print(&self) {
        println!("{:?}", self);
    }
}

trait SelfPrinter: Debug {
    fn print(&self) {
        println!("{:?}", self)
    }
}

trait AnotherPrinter {
    fn print() {
        println!("hello");
    }
}

#[derive(Debug)]
struct Bar;

impl FooPrinter for Bar {}

impl SelfPrinter for Bar {}

impl AnotherPrinter for Bar {}

#[test]
fn test_ufcs() {
    <()>::print(&());
    <Foo>::print(&Foo);
    Foo.print();
    let t: (i32, u16) = (3, 4);
    <(i32, u16)>::print(&t);
    t.print();
    // <Bar>::print();
    <Bar as AnotherPrinter>::print();
    <Bar as SelfPrinter>::print(&Bar);
}


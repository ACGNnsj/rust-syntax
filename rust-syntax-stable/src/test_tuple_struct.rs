use std::ops::Deref;
#[derive(Debug)]
struct MyBox<T>(T,T);
impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
impl <T:Copy> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x,x)
    }
}

#[test]
fn test() {
    let mut x = 5;
    let raw = &mut x;
    *raw = 6;
    println!("x = {}", x);
    let boxed = MyBox::new(x);
    println!("boxed = {:?}", boxed);
    let b=MyBox(1,2);
    println!("b = {:?}", b);
    println!("b.0 = {}", b.0);
    println!("*b = {}", *b);
}
#[derive(Debug)]
struct Bb(i32,i32);
#[test]
fn test2() {
    let bb=Bb(1,2);
    println!("bb = {:?}", bb);
}
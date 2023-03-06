#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}

/*impl Clone for Foo {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Copy for Foo {}*/


#[test]
fn test_drop() {
    { let foo = Foo; }
    println!("test_drop");
    let foo = Foo;
    println!("{:?}", foo);
    //Only use is to move foo in order to call drop of foo
    //If Copy trait is implemented for Foo, foo won't be moved
    core::mem::drop(foo);
    // println!("{:?}", foo);
}

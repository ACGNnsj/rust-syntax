#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Foo
    }
}

// impl Copy for Foo {}

fn my_drop(foo: Foo) {
    println!("still not dropped");
}

#[test]
fn test_drop() {
    { let foo = Foo; }
    println!("test_drop");
    let foo = Foo;
    println!("{:?}", foo);
    //Only use is to move foo in order to destruct foo once and only once at the end of core::mem::drop
    //If Copy trait is implemented for Foo, foo won't be moved
    core::mem::drop(foo);
    // println!("{:?}", foo);
    println!("start");
    let foo = Foo;
    my_drop(foo);
    println!("end");
    let foo = Foo;
    let foo2 = foo;
    println!("only dropped once");
}

#[test]
fn test_drop2() {
    let foo = Foo;
    let foo = Foo;
    println!("drop shadowed variable also");
}
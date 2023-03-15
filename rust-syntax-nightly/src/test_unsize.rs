use std::any::Any;
use std::marker::{PhantomData, Unsize};

fn is_sized<T: Sized>(s: T) {
    println!("sized");
}

struct TestStruct<T: Unsize<U> + ?Sized, U: ?Sized> {
    p: PhantomData<U>,
    a: T,
}
// impl Unsize<TestStruct> for i32 {}

trait TestTrait {
    fn test(&self);
}

impl TestTrait for TestStruct<[i32; 3], [i32]> {
    fn test(&self) {
        println!("test");
    }
}

struct UnsizeTestStruct<TestTrait: ?Sized> {
    sized: i32,
    a: TestTrait,
    // a: Box<TestTrait>,
    // bad: i32,
}


#[test]
fn test() {
    is_sized(1);
    is_sized(TestStruct::<[i32; 3], [i32]> { p: Default::default(), a: [1, 2, 3] });
    is_unsize::<[i32; 3], [i32]>(&[1, 2, 3]);
    is_unsize::<TestStruct<[i32; 3], [i32]>, dyn TestTrait>(&TestStruct { p: Default::default(), a: [1, 2, 3] });
    is_unsize::<UnsizeTestStruct<[i32; 3]>, UnsizeTestStruct<[i32]>>(&UnsizeTestStruct { sized: 0, a: [1, 2, 3] });
}

/*fn is_unsized<T: Unsize<T> + ?Sized>(s: &T) {
    println!("unsized");
}*/
fn is_unsize<T: Unsize<R> + ?Sized, R: Any + ?Sized>(s: &T) {
    println!("unsized");
}

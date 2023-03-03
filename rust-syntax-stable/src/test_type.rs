use std::ffi::OsString;

#[test]
fn test_type() {
    let mut a = 1;
    let b: *mut i32 = &mut a;
    println!("{:?}", b);
    let c: &mut i32 = &mut a;
    println!("{:?}", c);
    // let p: dyn ::std::ops::Deref<Target=i32>= OsString::new();
    let b=Box::new(1);
}
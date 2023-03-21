use std::any::type_name;
use std::ffi::OsString;
use std::fmt::Display;
use std::ops::{DivAssign, Mul, Neg};
use num::{FromPrimitive, Integer, NumCast};

fn change_array<T: Integer + NumCast + FromPrimitive + Display, const N: usize>(mut arr: [T; N]) {
    // let a = 10.into();
    arr[0] = T::zero();
    arr[1] = T::from_f32(2.5).unwrap();
    arr[N - 1] = T::from(11.8).unwrap();
    println!("{}", type_name::<T>());
    for i in &arr {
        println!("{}", i);
    }
    println!("{}", arr[N - 1]);
}

fn multiply<T>(a: T, b: T) where T: Mul<Output=T> + Copy + Display {
    println!("{}", a * b);
}

#[test]
fn test_type() {
    let mut a = 1;
    let b: *mut i32 = &mut a;
    println!("{:?}", b);
    let c: &mut i32 = &mut a;
    println!("{:?}", c);
    // let p: dyn ::std::ops::Deref<Target=i32>= OsString::new();
    let b = Box::new(1);
    let s = [1, 2, 3];
    change_array(s);
    multiply(5, 2);
    // println!("{}",5.3*2);
}
/*fn call_fn<T>(f: T) where T: Fn<()> {
    f();
}*/


use core::intrinsics::type_name;
use std::fmt::DebugTuple;

fn call_fn<T>(f: T) where T: Fn() {
    println!("call_fn begin");
    f();
    f();
    println!("call_fn end");
}

fn call_fn_mut<T: FnMut()>(mut f: T) {
    println!("call_fn_mut begin");
    f();
    f();
    println!("call_fn_mut end");
}

fn call_fn_once<T: FnOnce() -> () + Copy>(f: T) {
    println!("call_fn_once begin");
    f();
    f();
    println!("{}", type_name::<T>());
    println!("call_fn_once end");
}

fn call_fn_mut_2<T>(mut f: T) where T: FnMut(i32) -> i32 {
    println!("call_fn_mut begin");
    println!("{:?}", f(1));
    println!("{:?}", f(1));
    println!("call_fn_mut end");
}

fn call_fn_once_2<T: FnOnce(i32) -> (i32) + Copy>(f: T) {
    println!("call_fn_once begin");
    println!("{:?}", f(1));
    println!("{:?}", f(1));
    println!("{}", type_name::<T>());
    println!("call_fn_once end");
}

#[test]
fn test_0() {
    let e = 1;
    let f = || println!("{}", e);
    f();
    call_fn(f);
    Fn::call(&f, ());
    call_fn_mut(f);
    call_fn_once(f);
    let function = function_0;
    call_fn(function);
    call_fn_mut(function);
    call_fn_once(function);
    println!("{}", type_name_of(f));
    println!("{}", type_name_of(function));
    println!("{}", type_name::<dyn FnMut<(), Output=()>>());
    println!("{}", type_name::<DebugTuple>());
    let closure = move |x: i32| x + 1;
    call_fn_once_2(closure);
    call_fn_mut_2(closure);
}

fn function_0() {
    println!("function");
}

fn type_name_of<T>(_: T) -> &'static str {
    unsafe { type_name::<T>() }
}

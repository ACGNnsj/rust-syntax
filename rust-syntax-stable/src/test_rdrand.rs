use std::arch::x86_64::{_rdrand64_step, _rdseed64_step};

#[test]
fn test() {
    let mut v: u64 = 0;
    let mut value = 0;
    unsafe {
        let i = _rdrand64_step(&mut v);
        println!("{}", i);
        let i = _rdseed64_step(&mut value);
        println!("{}", i);
    }
    println!("{}", v);
    println!("{}", value);
}
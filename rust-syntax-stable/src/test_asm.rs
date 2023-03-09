use std::arch::asm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn add(mut a: i32, b: i32) -> i32 {
    unsafe {
        asm!("add {a}, {b}",
        a = inout(reg) a,
        b = in(reg) b,
        );
    }
    a
}

#[test]
fn test_asm() {
    print!("{} + {} = {}", 1, 2, add(1, 2));
    // let mut x: u64 = 4;
    // unsafe {
    //     asm!(
    //     "mov {tmp}, {x}",
    //     "shl {tmp}, 1",
    //     "shl {x}, 2",
    //     "add {x}, {tmp}",
    //     x = inout(reg) x,
    //     tmp = out(reg) _,
    //     );
    // }
    // assert_eq!(x, 4 * 6);
}
#[test]
fn test_raw_pointer() {
    let mut x = 5;
    let raw = &mut x as *mut i32;
    unsafe {
        *raw = 6;
    }
    let points_at = unsafe { *raw };
    assert_eq!(6, points_at);
    let raw = &mut x as *const i32;
    unsafe {
        println!("{}", *raw);
    }
}




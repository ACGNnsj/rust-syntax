unsafe fn local() {
    static mut COUNT: i32 = 0;
    print!("{} ", COUNT);
    COUNT = COUNT + 1;
}

#[test]
fn main() {
    unsafe {
        local();
        local();
        local();
    }
}
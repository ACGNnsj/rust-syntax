#[test]
fn test_never() {
    // let n = core::panicking::panic_fmt(format_args!("n"));
    // let d: i32 = n;
    let never = panic!("Never");
    // let a: () = (never,);
    let a: () = never;
    let n = { return a; };
}
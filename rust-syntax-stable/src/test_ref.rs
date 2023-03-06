fn change(i: &mut i32) {
    *i = 999;
}

fn reset(ref mut i: i32) {
    println!("{}", i);
    *i = 0;
    println!("{}", i);
}

#[test]
fn test_ref() {
    let mut i = 1;
    println!("{}", i);
    change(&mut i);
    println!("{}", i);
    reset(i);
    println!("{}", i);
}
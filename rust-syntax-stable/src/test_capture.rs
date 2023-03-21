#[test]
fn test() {
    let mut b = false;
    let a = &mut b;
    let mut c = || *a = true;
    // let y = &a;
    // println!("{}", a);
    c();
    let z = &a;
    println!("{}", a);
    println!("{}", b);
}

#[test]
fn test_2() {
    let mut b = false;
    let a = &mut b;
    let mut c = || *a == true;
    let y = &a;
    println!("{}", a);
    c();
    let z = &a;
    println!("{}", a);
    println!("{}", b);
}
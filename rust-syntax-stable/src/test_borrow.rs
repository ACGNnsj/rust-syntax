#[test]
fn test_borrow() {
    let mut a = 1;
    let b = &mut a;
    let c = &a;
    // *b = 3;
    // println!("{}", b);
    println!("{}", a);
}

#[test]
fn test_borrow2() {
    let mut a = 1;
    let b = &a;
    let c = &mut a;
    // println!("{}", b);
}

#[test]
fn test_borrow3() {
    let mut a = 1;
    let b = &mut a;
    let c = &mut a;
    // println!("{}", b);
    println!("{}", c);
}

#[test]
fn test_borrow4() {
    let mut a = String::from("some");
    let c = &a;

    let other = a; // 可能么?

    // println!("{}", c);
    // println!("{}", a);
    println!("{}", other);
}

#[test]
fn test_borrow5() {
    let mut a = 1;
    let b = &mut a;
    let c: i32 = a;
    println!("{}", a);
    // core::i32::MIN;
    // std::i32::MIN;
    // i32::MIN;
}

#[test]
fn test_borrow6() {
    let mut v = vec![1, 2, 3];
    let e = &mut v[0];
    *e = 4;
    println!("{:?}", v);
    v[1] = 11;
    println!("{:?}", v);
}
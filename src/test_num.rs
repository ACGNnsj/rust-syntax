#[test]
fn test_infinity() {
    let inf: i32 = f32::INFINITY as i32;
    println!("{}", inf);
    println!("{}", f32::INFINITY);
    println!("{}", f32::INFINITY as i32 == i32::MAX);
    println!("{}", (-3) / 2);
}

#[test]
fn test_division() {
    // println!("{}", positive_ceiling_division(1, 2));
    // println!("{}", positive_ceiling_division(0, 2));
    // println!("{}", positive_ceiling_division(2, 2));
    // println!("{}", positive_ceiling_division(-1, 2));
    println!("{}", positive_ceiling_division(18, 3));
    println!("{}", positive_ceiling_division(18, 4));
    println!("{}", positive_ceiling_division(18, 5));
    println!("{}", positive_ceiling_division(18, 6));
    println!("{}", positive_ceiling_division(18, 7));
    println!("{}", positive_ceiling_division(18, 8));
    println!("{}", positive_ceiling_division(18, 9));

    println!("{}", 18/3);
    println!("{}", 18/4);
    println!("{}", 18/5);
    println!("{}", 18/6);
    println!("{}", 18/7);
    println!("{}", 18/8);
    println!("{}", 18/9);
    
}

pub fn positive_ceiling_division(a: i32, b: i32) -> i32 {
    (a - 1) / b + 1
}
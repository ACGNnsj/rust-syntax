#[test]
fn test_some(){
    let x = 5;
    let y: Option<i32> = Some(x);
    println!("y: {:?}", y);
}
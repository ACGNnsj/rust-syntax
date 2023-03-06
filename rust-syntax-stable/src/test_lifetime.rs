#[test]
fn test_lifetime(){
    let r;

    {
        let x = 5;
        // r = &x;
        r = x;
    }

    println!("r: {}", r);
}
#[test]
fn test_str(){
    let mut s1 = "Hello,".to_string();
    let s2 = "world".to_string();
    s1 += &s2;
    assert_eq!(s1, "Hello,world");
    println!("s1 = {}", s1);
    let s3="wad";
    s1+=s3;
    println!("s3");
    println!("{}",s1);
    return ();
}
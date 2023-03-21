#[test]
fn test() {
    let s: String = <&str>::into("ttgt");
    println!("{}", s);
    let s: String = "ttgt".into();
    println!("{}", s);
    let str: String = String::from("birthday gift").into_boxed_str().into_string();
    println!("{}", str);
    
}
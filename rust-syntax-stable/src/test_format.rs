#[test]
fn test_format() {
    let person = "Rustaceans";
    let msg = "Hello";
    println!("{} {}", msg, person);
    println!("{0} {1} {0}", msg, person);
    println!("{msg} {person}", msg = msg, person = person);
    println!("{msg} {person}");
    let name = "Peter";
    let score = 13454530.63342243;
    let (width, precision) = (25, 3);
    println!("Hello, {name}! Your score is {score:width$.precision$}", name = "Doe");
}
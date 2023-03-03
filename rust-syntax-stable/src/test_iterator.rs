use std::ops::Index;

#[test]
fn test_iterator() {
    let str = "零一二三四五六七八九十终";
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{}: {}", i, item);
    }
    println!("{}", first_word("Hello, world"));
    println!("{}", first_word("Hello"));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let result = s.index(0..i);
            // let result = &s[0..i];
            return result;
        }
    }
    s
    // &s[..]
}
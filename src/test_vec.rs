#[test]
fn test_max() {
    let nums = vec![1, 2, 7, 3, 4, 5, 7, 7];
    let max = nums.iter().max().unwrap();
    let max_index = nums.iter().position(|&x| x == *max).unwrap();
    let all_max = nums.iter().filter(|&&x| x == *max).collect::<Vec<&i32>>();
    let all_max_index = nums.iter().enumerate().filter(|&(_, &x)| x == *max).map(|(i, _)| i).collect::<Vec<usize>>();
    println!("max: {}, all_max_index: {:?}", max, all_max_index);
}

use std::ops::Index;

#[test]
fn test_vec() {
    let string_vec: Vec<String> = vec![
        String::from("hello"),
        String::from("world"),
    ];
    // let a = string_vec[0];
    // let a = *string_vec.index(0);
    let a = string_vec.index(0);
    println!("{}", a);
    let b = &string_vec[1];
    println!("{}", b);
}
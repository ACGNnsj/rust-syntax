#[test]
fn test_some() {
    let some_u8_value: Option<i32> = Some(3);
    let s = some_u8_value.unwrap();
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("other");
    }
    match s {
        3 => println!("3"),
        _ => println!("other")
    }
    let list = [21];
    match &list[..] {
        [] => println!("empty"),
        [_] => println!("single"),
        [x, inside @ .., y] if x == y => println!("{:?}", inside),
        _ => println!("other")
    }
    let list = [1, 3, 5, 4, 3, 1];
    println!("{}", is_symmetric(&list));
}

fn is_symmetric<T: PartialEq>(list: &[T]) -> bool {
    /*if list.len() < 2 {
        return true;
    }
    let (first, last) = list.split_at(1);
    let (inside, last) = last.split_at(last.len() - 1);
    first == last && is_symmetric(inside)*/
    match list {
        [] | [_] => true,
        [x, inside @ .., y] if x == y => is_symmetric(inside),
        _ => false
    }
}




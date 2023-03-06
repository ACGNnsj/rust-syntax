// use std::collections::BTreeMap;
extern crate alloc;

use alloc::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum JsonElement {
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonElement>),
    Object(BTreeMap<String, JsonElement>),
}

fn is_json(element: &JsonElement) -> bool {
    match element {
        JsonElement::Boolean(_) => false,
        JsonElement::Number(_) => false,
        JsonElement::String(_) => false,
        JsonElement::Array(_) => false,
        JsonElement::Object(_) => true,
    }
}

#[test]
fn test_json() {
    let mut json = JsonElement::Object(BTreeMap::new());
    println!("{:?}", json);
    println!("{}", is_json(&json));
    match json {
        JsonElement::Object(ref mut map) => {
            map.insert("key".to_string(), JsonElement::String("value".to_string()));
        }
        _ => {}
    }
    println!("{:?}", json);
    println!("{}", is_json(&json));
}
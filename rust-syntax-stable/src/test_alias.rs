#[derive(Debug)]
struct OriginType;

type AliasType = OriginType;

#[derive(Debug)]
struct AnotherOriginType;

use std::fmt::Debug;
use AnotherOriginType as AnotherAliasType;

#[test]
fn test() {
    let origin: AliasType = OriginType;
    println!("origin: {:?}", origin);
    // let alias = AliasType;
    let another_origin: AnotherAliasType = AnotherOriginType;
    println!("another_origin: {:?}", another_origin);
    let another_alias: AnotherOriginType = AnotherAliasType;
    println!("another_alias: {:?}", another_alias);
    let x: <OriginType as TypeBound>::BoundType = AnotherAliasType;
    // println!("alias_type_name: {:?}", std::intrinsics::type_name::<AliasType>());
    println!("alias_type_name: {:?}", std::any::type_name::<AliasType>());
    println!("another_alias_type_name: {:?}", std::any::type_name::<AnotherAliasType>());
    println!("bound_type_name: {:?}", std::any::type_name::<<AliasType as TypeBound>::BoundType>());
}

trait TypeBound {
    type BoundType: Debug;
}

impl TypeBound for AliasType {
    type BoundType = AnotherAliasType;
}
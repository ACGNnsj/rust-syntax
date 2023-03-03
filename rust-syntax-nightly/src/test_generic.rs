
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn display_array_slice<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

#[test]
fn test_generic() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    display_array_slice(&arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
    display_array_slice(&arr);
}

struct ConstDefault;

impl const Default for ConstDefault {
    fn default() -> Self {
        Self
    }
}

struct NonConstDefault;

impl Default for NonConstDefault {
    fn default() -> Self {
        Self
    }
}

const fn foo<T: ~ const Default>() -> T {
    T::default()
}

use core::default::default;

#[test]
fn test_const_trait() {
    // const foo implies const Default = true
    const _FOO: ConstDefault = foo();
    let a = default::<ConstDefault>();
    let b:NonConstDefault = default();
    // println!("{}", _);
    println!("Hello, world!");
    // const foo implies !const Default = false
    // const _BAR: NonConstDefault = foo();
    // ^ uncomment for compile error

    // !const foo implies const Default = true
    let _a: ConstDefault = foo();

    // !const foo implies !const Default = true
    let _b: NonConstDefault = foo();
}

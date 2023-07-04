use std::collections::HashMap;

#[test]
fn test(){
    // let arr= crate::array_impl!{ "abc",arr};
    // print!("{:?}",arr);
}

#[macro_export]
macro_rules! array_impl {
    {$n:expr, $t:ident $($ts:ident)*} => {
        impl<T> const Default for [T; $n] where T: ~const Default {
            fn default() -> [T; $n] {
                [$t::default(), $($ts::default()),*]
            }
        }
        array_impl_default!{($n - 1), $($ts)*}
    };
    {$n:expr,} => {
        impl<T> const Default for [T; $n] {
            fn default() -> [T; $n] { [] }
        }
    };
}
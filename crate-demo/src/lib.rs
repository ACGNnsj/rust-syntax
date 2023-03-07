mod test;
mod test_mod;
use core::ops;
// use crate::ops::Add;
// use crate::test_mod::ops::Add;
use crate::test::ops::Add;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        crate::ops::Add::add(1, 2);
    }
}

use crate::test_mod::sub::Sa;


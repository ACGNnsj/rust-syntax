#![feature(unsize)]
#![feature(const_trait_impl)]
#![feature(default_free_fn)]
#![feature(inline_const)]
// #![allow(soft_unstable)]
#![feature(test)]
// #![feature(panic_abort)]
// #[cfg(feature="nightly")]
// #[cfg(nightly)]
mod test_generic;
mod test_unstable_library;
// #[cfg(test)]
mod test_benchmark;
mod test_unsize;

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
    }
}

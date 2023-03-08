use public_function::prime::count_prime;

#[test]
fn test() {
    let t0 = std::time::Instant::now();
    count_prime(10000000);
    let t1 = std::time::Instant::now();
    println!("Time elapsed: {:?}", t1 - t0);
}

extern crate test;

use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    // assert_eq!(count_prime_numbers(100), 25);
    b.iter(|| count_prime(10000));
}


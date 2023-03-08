pub fn count_prime(n: u32) -> u32 {
    match n {
        0 | 1 => return 0,
        2 => return 1,
        3 | 4 => return 2,
        5 | 6 => return 3,
        7 | 8 | 9 | 10 => return 4,
        _ => (),
    }
    let mut count = 4;
    let mut i = 11;
    while i <= n {
        if is_prime_inner(i) {
            count += 1;
            // println!("{} is prime", i);
        }
        i += 2;
    }
    count
}

fn is_prime_inner(n: u32) -> bool {
    let sqrt = (n as f32).sqrt() as u32;
    let mut i = 3;
    while i <= sqrt {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

pub fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 | 4 | 6 | 8 | 9 | 10 => return false,
        2 | 3 | 5 | 7 => return true,
        _ => (),
    }
    let sqrt = (n as f32).sqrt() as u32;
    let mut i = 3;
    while i <= sqrt {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}


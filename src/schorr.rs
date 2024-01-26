// src/schorr.rs
// use num_bigint::BigUint;
use num_bigint_dig::{BigUint, RandBigInt};
use rand::{thread_rng, Rng};

pub fn rand_gen() -> [u8; 32] {
    let mut rng = thread_rng();
    let mut bytes = [0u8; 32]; // 256 bits
    rng.fill(&mut bytes);
    bytes[0] |= 0b10000000; // set MSB to 1 is 8 bits because needs to or with u8 (8 bits) used to ensure that the number has appropriate entropy
    bytes[bytes.len() - 1] |= 0b00000001; // same as above but LSB but for increase prob its prime
    bytes
}

pub fn array_to_bigint(bytes: &[u8]) -> BigUint {
    let concatenated_string = bytes.iter().map(|&num| num.to_string()).collect::<String>();
    concatenated_string.parse::<BigUint>().unwrap()
}

pub fn miller_rabin(n: &BigUint, k: u32) -> bool {
    let mut rng = thread_rng();
    let mut r = 0;
    let mut d = n - BigUint::from(1 as u8);

    while &d % BigUint::from(2 as u8) == BigUint::from(0 as u8) {
        r += 1;
        d /= &BigUint::from(2 as u8);
    }

    for _ in 0..k {
        let a: BigUint =
            rng.gen_biguint_range(&BigUint::from(2 as u8), &(n - &BigUint::from(1 as u8)));
        let mut x = a.modpow(&d, &n);

        if x != BigUint::from(1 as u8) && x != n - BigUint::from(1 as u8) {
            let mut j = 1;
            while j < r && x != (n - &BigUint::from(1 as u8)) {
                x = a.modpow(&BigUint::from(2 as u8), &n);
                if x != BigUint::from(1 as u8) && x != n - BigUint::from(1 as u8) {
                    return false;
                }
                j += 1;
            }
            if x != n - BigUint::from(1 as u8) {
                return false;
            }
        }
    }
    true
}

pub fn gen_prime() -> BigUint{
    loop {
        let rand_num = &array_to_bigint(&rand_gen());
        let is_prime = miller_rabin(&rand_num, 128);
        println!("{}: {}", is_prime, rand_num);
        if is_prime {
            return rand_num.clone()
        }
    }
}

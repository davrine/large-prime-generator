// src/schorr.rs
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use rand::{thread_rng, Rng};

pub fn rand_gen() -> [u8; 32] {
    let mut rng = thread_rng();
    let mut bytes = [0u8; 32]; // 256 bits
    rng.fill(&mut bytes);
    bytes[0] |= 0b10000000; // set MSB to 1 is 8 bits because needs to or with u8 (8 bits) used to ensure that the number has appropriate entropy
    bytes[bytes.len() - 1] |= 0b00000001; // same as above but LSB
    bytes
}

pub fn array_to_bigint(arr: &[u8]) -> BigUint {
    arr.iter()
        .fold(BigUint::from_u8(0).unwrap(), |acc, &digit| {
            acc * 10u8 + BigUint::from_u8(digit).unwrap()
        })
}

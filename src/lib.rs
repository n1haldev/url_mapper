use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero, ToPrimitive};

const CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn base62_encode(num: BigUint) -> String {
    let base: BigUint = 62.to_biguint().unwrap();
    let mut encoded = String::new();

    let all_chars: Vec<char> = CHARS.chars().collect();

    let mut num = num;

    while num > BigUint::zero() {
        let remainder = (&num % &base).to_usize().unwrap();
        encoded.push(all_chars[remainder]);
        num = &num / &base; 
    }

    encoded.chars().rev().collect()
}

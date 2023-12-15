use num_bigint::BigUint;
use num_traits::{pow, ToPrimitive};
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(2..p);
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let bg: BigUint = BigUint::from(g);
    let bp: BigUint = BigUint::from(p);
    return (pow(bg, a as usize) % bp).to_u64().unwrap();
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let bb: BigUint = BigUint::from(b_pub);
    let bp: BigUint = BigUint::from(p);
    return (pow(bb, a as usize) % bp).to_u64().unwrap();
}

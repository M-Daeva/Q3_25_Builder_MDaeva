pub fn calc_sqrt(a: u64, b: u64) -> u64 {
    (a as u128 * b as u128).isqrt() as u64
}

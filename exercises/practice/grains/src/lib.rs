const MIN_SQUARE: u64 = 1;
const MAX_SQUARE: u64 = 64;
const QUOTIENT: u64 = 2;

pub fn square(square_number: u32) -> u64 {
    if !(MIN_SQUARE..=MAX_SQUARE).contains(&(square_number as u64)) {
        panic!("Square must be between {} and {}", MIN_SQUARE, MAX_SQUARE);
    }
    QUOTIENT.pow(square_number - 1)
}

pub fn total() -> u64 {
    let a1 = MIN_SQUARE as u128;
    let n = MAX_SQUARE as u32;

    (a1 * ((QUOTIENT as u128).pow(n) - 1) / (QUOTIENT as u128 - 1)) as u64
}

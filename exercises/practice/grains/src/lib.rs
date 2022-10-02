const SQUARE_OUT_OF_RANGE: &str = "Square must be between 1 and 64";
const MIN_SQUARE: u64 = 1;
const MAX_SQUARE: u64 = 64;
const QUOTIENT: u64 = 2;

pub fn square(s: u32) -> u64 {
    if !(MIN_SQUARE..=MAX_SQUARE).contains(&(u64::from(s))) {
        panic!("{}", SQUARE_OUT_OF_RANGE);
    }
    QUOTIENT.pow(s - 1)
}

pub fn total() -> u64 {
    let a1 = u128::from(MIN_SQUARE);
    let n = MAX_SQUARE as u32;

    (a1 * (u128::from(QUOTIENT).pow(n) - 1) / u128::from(QUOTIENT - 1)) as u64
}

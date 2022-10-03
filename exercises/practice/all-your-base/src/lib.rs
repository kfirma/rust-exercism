#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///

const MIN_BASE: u32 = 2;

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    check_parameters(number, from_base, to_base)?;

    if number.is_empty() {
        return Ok(vec![0]);
    }

    // digits to base 10 number
    let mut number_as_base_10 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(index, value)| value * from_base.pow(index as u32))
        .sum::<u32>();

    if number_as_base_10 == 0 {
        return Ok(vec![0]);
    }

    // convert to target base
    let mut new_number = Vec::new();
    while number_as_base_10 > 0 {
        new_number.insert(0, number_as_base_10 % to_base);
        number_as_base_10 /= to_base;
    }

    Ok(new_number)
}

fn check_parameters(number: &[u32], from_base: u32, to_base: u32) -> Result<(), Error> {
    if from_base < MIN_BASE {
        return Err(Error::InvalidInputBase);
    }

    if to_base < MIN_BASE {
        return Err(Error::InvalidOutputBase);
    }

    if let Some(&digit) = number.iter().find(|&&digit| digit >= from_base) {
        return Err(Error::InvalidDigit(digit));
    }

    Ok(())
}

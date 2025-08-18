use core::num;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn base_to_decimal(digits: &[u32], from_base: u32) -> Result<u32, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    let mut decimal = 0;
    digits.iter().rev().enumerate().try_for_each(|(idx, el)| {
        if el >= &from_base {
            return Err(Error::InvalidDigit(*el));
        }
        decimal += *el * from_base.pow(idx as u32);
        Ok(())
    })?;
    Ok(decimal)
}

fn decimal_to_base(decimal: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    let mut decimal = decimal;
    let mut ans = vec![];
    while decimal > 0 {
        ans.push(decimal % to_base);
        decimal /= to_base;
    }
    ans.reverse();
    Ok(ans)
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
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
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // cleanup leading zeroes
    let start = number.iter().position(|&x| x != 0).unwrap_or(number.len());
    let number = &number[start..];
    
    let decimal = base_to_decimal(number, from_base)?;
    if number.is_empty() {
        return Ok(vec![0]);
    }
    decimal_to_base(decimal, to_base)
}

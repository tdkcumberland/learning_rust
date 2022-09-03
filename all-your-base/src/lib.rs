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
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {

    if from_base == 1 || from_base == 0 {
        return Err(Error::InvalidInputBase)
    } else if to_base == 1 || to_base == 0 {
        return Err(Error::InvalidOutputBase)
    } else if number.len() == 0 {
        return Ok(vec![0])
    }

    let accumulator = get_total_from(number, from_base);
    match accumulator {
        Ok(acc) => produce_converted(acc, to_base as i32),
        Err(err) => Err(err)
    }
    
}

fn get_total_from(number: &[u32], from_base: u32) -> Result<i32, Error> {
    let len = number.len();
    let mut accumulator: u32 = 0;
    for n in (0..=len-1).rev(){
        let digit = number[len-1-n];
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit))
        }
        accumulator = accumulator + digit * from_base.pow(n as u32);
        
    };
    Ok(accumulator as i32)
}

fn produce_converted(accumulator: i32, to_base: i32) -> Result<Vec<u32>, Error> {
    let mut b = vec![];
    let mut remainder = accumulator / to_base;
    let mut modulus = accumulator as i32 - remainder * to_base;
    b.push(modulus as u32);
    while remainder > 0  {
        let current = remainder;
        remainder = remainder  / to_base ;
        modulus = current as i32 - remainder * to_base;
        b.insert(0, modulus as u32)
    }
    Ok(b)
}

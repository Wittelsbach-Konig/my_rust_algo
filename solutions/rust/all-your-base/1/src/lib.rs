#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut num = 0_u32;
    for &n in number {
        if n >= from_base {
            return Err(Error::InvalidDigit(n));
        }
        num = num * from_base + n;
    }

    if num == 0 {
        return Ok(vec![0]);
    }
    let mut result: Vec<u32> = Vec::new();
    while num > 0 {
        result.push(num % to_base);
        num /= to_base;
    }
    result.reverse();
    Ok(result)
}
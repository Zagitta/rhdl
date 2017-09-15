use std::iter::Iterator;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use std::str::FromStr;

pub trait FromIter<T, E> where Self: Sized {
    fn from_iter<I: Iterator<Item=T>>(iter: I) -> Result<Self, E>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseIntError {
    Empty, 
    InvalidDigit, 
    Overflow, 
    Underflow, 
}


macro_rules! impl_char_digit_parser {
    ($T:ident) => {
        impl FromIter<char, ParseIntError> for $T {
            fn from_iter<I: Iterator<Item=char>>(mut iter: I) -> Result<Self, ParseIntError> {
                let (positive, mut res) = match iter.next() {
                    Some(c) => match c {
                        '+' => (true, $T::default()),
                        '-' => (false, $T::default()),
                        _ => match c.to_digit(10) {
                            Some(d) => (true, d as $T),
                            None => return Err(ParseIntError::InvalidDigit)
                        }
                    },
                    None => return Err(ParseIntError::Empty)
                };

                if positive {
                    for c in iter {
                    let x = match c.to_digit(10) {
                            Some(d) => d as $T,
                            None => return Err(ParseIntError::InvalidDigit) 
                        };
                        res = match res.checked_mul(10) {
                            Some(res) => res,
                            None => return Err(ParseIntError::Overflow)
                        };
                        res = match res.checked_add(x) {
                            Some(res) => res,
                            None => return Err(ParseIntError::Overflow)
                        };
                    }
                } else {
                    for c in iter {
                    let x = match c.to_digit(10) {
                            Some(d) => d as $T,
                            None => return Err(ParseIntError::InvalidDigit) 
                        };
                        res = match res.checked_mul(10) {
                            Some(res) => res,
                            None => return Err(ParseIntError::Underflow)
                        };
                        res = match res.checked_sub(x) {
                            Some(res) => res,
                            None => return Err(ParseIntError::Underflow)
                        };
                    }
                }

                Ok(res)
            }
        } 
    }
}


macro_rules! impl_u8_digit_parser {
    ($T:ident) => {
        impl FromIter<u8, ParseIntError> for $T {
            fn from_iter<I: Iterator<Item=u8>>(mut iter: I) -> Result<Self, ParseIntError> {
                let (positive, mut res) = match iter.next() {
                    Some(c) => match c {
                        b'+' => (true, $T::default()),
                        b'-' => (false, $T::default()),
                        _ => match (c as char).to_digit(10) {
                            Some(d) => (true, d as $T),
                            None => return Err(ParseIntError::InvalidDigit)
                        }
                    },
                    None => return Err(ParseIntError::Empty)
                };

                if positive {
                    for c in iter {
                    let x = match (c as char).to_digit(10) {
                            Some(d) => d as $T,
                            None => return Err(ParseIntError::InvalidDigit) 
                        };
                        res = match res.checked_mul(10) {
                            Some(res) => res,
                            None => return Err(ParseIntError::Overflow)
                        };
                        res = match res.checked_add(x) {
                            Some(res) => res,
                            None => return Err(ParseIntError::Overflow)
                        };
                    }
                } else {
                    for c in iter {
                    let x = match (c as char).to_digit(10) {
                            Some(d) => d as $T,
                            None => return Err(ParseIntError::InvalidDigit) 
                        };
                        res = match res.checked_mul(10) {
                            Some(res) => res,
                            None => return Err(ParseIntError::Underflow)
                        };
                        res = match res.checked_sub(x) {
                            Some(res) => res,
                            None => return Err(ParseIntError::Underflow)
                        };
                    }
                }

                Ok(res)
            }
        } 
    }
}

impl_char_digit_parser!(i8);
impl_char_digit_parser!(i16);
impl_char_digit_parser!(i32);
impl_char_digit_parser!(i64);
impl_char_digit_parser!(u8);
impl_char_digit_parser!(u16);
impl_char_digit_parser!(u32);
impl_char_digit_parser!(u64);


impl_u8_digit_parser!(i8);
impl_u8_digit_parser!(i16);
impl_u8_digit_parser!(i32);
impl_u8_digit_parser!(i64);
impl_u8_digit_parser!(u8);
impl_u8_digit_parser!(u16);
impl_u8_digit_parser!(u32);
impl_u8_digit_parser!(u64);




#[cfg(test)]
mod tests {
    use super::*;
    use self::ParseIntError::*;

    #[test]
    fn fussy_parsing() {
        
    }

    #[test]
    fn parsing_invalid_digit_fails() {
        assert_eq!(i64::from_iter("asd".chars()), Err(InvalidDigit));
        assert_eq!(i64::from_iter("123123asd".chars()), Err(InvalidDigit));
        assert_eq!(i64::from_iter("+-a".chars()), Err(InvalidDigit));
    }

    #[test]
    fn parsing_overflows() {
        assert_eq!(i64::from_iter("12312331231253987192874998174".chars()), Err(Overflow));
    }
    #[test]
    fn parsing_underflows() {
        assert_eq!(i64::from_iter("-12312331231253987192874998174".chars()), Err(Underflow));
    }
    #[test]
    fn parsing_empty() {
        assert_eq!(i64::from_iter("".chars()), Err(Empty));
    }
}


#[cfg(feature = "bench")]
mod bench {
    extern crate test;
    use super::*;
    use self::test::Bencher;

    const DATA: &'static str = "112321237451111111116123";

    #[bench]
    fn bench_char_impl(b: &mut Bencher) {
        b.iter(|| {
            i64::from_iter(DATA.chars())
        })
    }

    #[bench]
    fn bench_u8_impl(b: &mut Bencher) {
        b.iter(|| {
            i64::from_iter(DATA.bytes())
        })
    }

    #[bench]
    fn bench_native(b: &mut Bencher) {
        b.iter(|| {
            i64::from_str(DATA)
        })
    }


    #[bench]
    fn bench_match(b: &mut Bencher) {
        b.iter(|| {
            let mut a: i64 = 0;
            for c in DATA.chars().filter(|c| *c != '_') {
                match c.to_digit(10) {
                    Some(d) => a = a * 10 + d as i64,
                    None => return None
                }
            }
            Some(a)
        });
    }


    #[bench]
    fn bench_map(b: &mut Bencher) {
        b.iter(|| {
            DATA.chars().filter(|c| *c != '_').map(|c| c.to_digit(10)).fold_options(0i64, |acc, d| acc * 10 + d as i64)
        });
    }

    #[bench]
    fn bench_naive(b: &mut Bencher) {
        b.iter(|| {
            i64::from_str(&DATA.replace("_", ""))
        });
    }
}

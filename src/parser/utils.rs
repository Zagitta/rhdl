use std::iter::Iterator;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use std::str::FromStr;

pub trait FromIter<I, E> where I : Iterator, Self: Sized {
    fn from_iter(iter: I) -> Result<Self, E>;
}

#[derive(Debug)]
pub enum ParseIntError {
    Empty, 
    InvalidDigit, 
    Overflow, 
    Underflow, 
}


impl<I: Iterator<Item=char>> FromIter<I, ParseIntError> for i64 {
    fn from_iter(iter: I) -> Result<Self, ParseIntError> {
        let mut i = iter.peekable();

        let (positive, advance) = match i.peek() {
            Some(c) => match *c {
                '+' => (true, true),
                '-' => (false, true),
                _ => (true, false)
            },
            None => return Err(ParseIntError::Empty)
        };

        if advance {
            let _ = i.next();
        }

        let mut res = 0i64;
        if positive {
            for c in i {
               let x = match c.to_digit(10) {
                    Some(d) => d as i64,
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
            for c in i {
               let x = match c.to_digit(10) {
                    Some(d) => d as i64,
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



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let tv = vec![
            "123123",
            "asda",
            "980789",
            "9789789678687634",
            "12312331231253987192874998174",
            "-123",
            "123123asd",
            "-a",
            "---",
            "+++",
            "+123",
            "+-a"
        ];

        let EMPTY = i64::from_str("").unwrap_err();
        let IDIGIT = i64::from_str("a").unwrap_err();
        let OF = i64::from_str("12312331231253987192874998174").unwrap_err();
        let UF = i64::from_str("-12312331231253987192874998174").unwrap_err();

        for s in &tv {
            let ri = i64::from_iter(s.chars());
            let rs = i64::from_str(s);
            
            use self::ParseIntError::*;
            
            match (ri, rs) {
                (Ok(lhs), Ok(rhs)) => assert_eq!(lhs, rhs),
                (Err(lhs), Err(rhs)) => {
                    assert!(match (lhs, rhs) {
                        (Empty, EMPTY) => true,
                        (InvalidDigit, IDIGIT) => true,
                        (Overflow, OF) => true,
                        (Underflow, UF) => true,
                        _ => false
                    });
                },
                _ => assert!(false)
            }
        }
    }
}


#[cfg(feature = "bench")]
mod bench {
    extern crate test;
    use super::*;
    use self::test::Bencher;

    const DATA: &'static str = "112321237451111111116123";

    #[bench]
    fn bench_impl(b: &mut Bencher) {
        b.iter(|| {
            i64::from_iter(DATA.chars())
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

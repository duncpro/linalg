#[derive(Clone)]
pub struct Rational { 
    numerator: usize,
    denominator: usize,
    sign: Sign 
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sign { Positive, Negative }

pub struct ParseRationalError;

impl Rational {
    pub fn new(sign: Sign, numerator: usize, denominator: usize) -> Self {
        assert!(denominator != 0, "division by zero is undefined");
        let mut rational = Rational { numerator, denominator, sign };
        reduce(&mut rational);
        return rational;
    }

    pub fn multiplicative_inverse(other: &Self) -> Self {
        assert!(other.denominator != 0, "zero has no multiplicative inverse");
        Rational::new(other.sign, other.denominator, other.numerator)
    }

    pub fn additive_inverse(other: &Self) -> Self {
        let mut rational = other.clone();
        if rational != Rational::zero() {
            flip_sign(&mut rational.sign);
        }     
        return rational;
    }

    pub fn add_inplace(&mut self, mut qty: Self) {      
        relativize(self, &mut qty);
        if self.sign == qty.sign { 
            self.numerator += qty.numerator;
        } else {
            if qty.numerator > self.numerator {
                let overflow = qty.numerator.saturating_sub(self.numerator);
                flip_sign(&mut self.sign);
                self.numerator = overflow;
            } else {
                self.numerator -= qty.numerator;
            }
        }
        reduce(self);
    }

    pub fn mul_inplace(&mut self, factor: &Self) {
        use Sign::{Positive, Negative};
        self.numerator *= factor.numerator;
        self.denominator *= factor.denominator;
        self.sign = match (self.sign, factor.sign) {
            (Positive, Positive) => Positive,
            (Positive, Negative) => Negative,
            (Negative, Positive) => Negative,
            (Negative, Negative) => Positive,
        };
        reduce(self);
    }

    pub fn numerator(&self) -> usize { self.numerator }
    pub fn denominator(&self) -> usize { self.denominator }

    pub fn zero() -> Self { Rational::new(Sign::Positive, 0, 1) }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if matches!(self.sign, Sign::Negative) {
            std::fmt::Write::write_char(f, '-')?;
        }
        write!(f, "{}", self.numerator)?;
        if self.denominator != 1 {
            write!(f, "/{}", self.denominator)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Rational {
    type Err = ParseRationalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars().peekable();
        let sign = match iter.next_if_eq(&'-').is_some() {
            true => Sign::Negative,
            false => Sign::Positive,
        };
        let mut numerator: usize = 0;
        while let Some(digit) = iter.next_if(|c| c.is_ascii_digit()) {
            numerator *= 10;
            numerator += digit as usize - 48 /* '0' */;
        }
        let mut denominator: usize = 0;
        if iter.next_if_eq(&'/').is_some() {
            while let Some(digit) = iter.next_if(|c| c.is_ascii_digit()) {
                denominator *= 10;
                denominator += digit as usize - 48 /* '0' */;
            }
        } else { 
            denominator = 1;
        }
        if iter.count() != 0 { return Err(ParseRationalError); }
        Ok(Rational::new(sign, numerator, denominator))
    }
}

impl std::cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator &&
        self.denominator == other.denominator &&
        self.sign == other.sign
    }
}

impl std::cmp::Eq for Rational {}

impl Default for Rational {
    fn default() -> Self { Rational::zero() }
}

impl From<isize> for Rational {
    fn from(value: isize) -> Self {
        if value == 0 { return Self::zero(); }
        let sign = if value > 0 { Sign::Positive } else { Sign::Negative };
        return Self::new(sign, value.unsigned_abs(), 1); 
    }
}

fn calc_gcf(mut x: usize, mut y: usize) -> usize {
    if x == 0 { panic!("calc_gcf expected positive integer x but encountered {}", x); }
    if y == 0 { panic!("calc_gcf expected positive integer y but encountered {}", y); }
    loop {
        use std::cmp::Ordering::{Less, Equal, Greater};
        match x.cmp(&y) {
            Less => { y = y - x; },
            Equal => { return x; },
            Greater => { x = x - y; },
        }
    }
}

fn reduce(num: &mut Rational) {
    if num.numerator == 0 {
        num.sign = Sign::Positive;
        num.denominator = 1; 
        return;
    }
    let gcf = calc_gcf(num.numerator, num.denominator);
    num.numerator /= gcf;
    num.denominator /= gcf;
}

fn relativize(a: &mut Rational, b: &mut Rational) {
    let a_denominator = a.denominator;
    
    a.numerator *= b.denominator;
    a.denominator *= b.denominator;

    b.numerator *= a_denominator;
    b.denominator *= a_denominator;
}

fn flip_sign(sign: &mut Sign) {
    *sign = match sign {
        Sign::Positive => Sign::Negative,
        Sign::Negative => Sign::Positive,
    }
}


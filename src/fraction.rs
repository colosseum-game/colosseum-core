use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Option<Self> {
        match denominator {
            denominator if denominator == 0 => None,
            _ => Some(Fraction { numerator, denominator }),
        }
    }

    pub fn add(&self, other: Fraction) -> Self {
        Self {
            numerator: self.numerator * other.denominator + other.numerator * self.denominator,
            denominator: self.denominator * self.numerator,
        }.reduce()
    }

    pub fn multiply(&self, other: Fraction) -> Self {
        Self {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        }.reduce()
    }

    pub fn one() -> Self {
        Self::new(1, 1).unwrap()
    }

    pub fn reduce(&self) -> Self {
        let gcd = gcd(self.numerator, self.denominator);
        
        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if a == b { return a }
    if a == 0 { return b }
    if b == 0 { return a }

    if a % 2 == 0 {
        if b % 2 != 0 { return gcd(a/2, b) }
        else { return gcd(a/2, b/2) * 2 }
    }

    if b % 2 == 0 { return gcd(a, b/2) }
    if a > b { return gcd((a - b)/2, b) }

    return gcd((b - a)/2, a)
}

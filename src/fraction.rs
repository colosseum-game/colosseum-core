pub use crate::fraction_generated::{
    file_descriptor,
    Fraction,
};

impl Fraction {
    pub fn add(&mut self, other: &Fraction) {
        self.numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        self.denominator = self.denominator * self.numerator;
        self.reduce();
    }

    pub fn multiply(&mut self, other: &Fraction) {
        self.numerator = self.numerator * other.numerator;
        self.denominator = self.denominator * other.denominator;
        self.reduce();
    }

    pub fn one() -> Fraction {
        let mut one = Fraction::new();
        one.numerator = 1;
        one.denominator = 1;
        one
    }

    pub fn reduce(&mut self) {
        let gcd = gcd(self.numerator, self.denominator);
        self.numerator = self.numerator / gcd;
        self.denominator = self.denominator / gcd;
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if a == b { return a }
    if a == 0 { return b }
    if b == 0 { return a }

    if a % 2 == 0 {
        if b % 2 != 0 { return gcd(a / 2, b) }
        else { return gcd(a / 2, b / 2) * 2 }
    }

    if b % 2 == 0 { return gcd(a, b / 2) }
    if a > b { return gcd((a - b) / 2, b) }

    return gcd((b - a) / 2, a)
}

#[derive(Clone, Copy, Debug)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
}

impl Fraction {
    pub fn add(&self, other: Fraction) -> Fraction {
        Fraction {
            numerator: self.numerator * other.denominator + other.numerator * self.denominator,
            denominator: self.denominator * other.denominator
        }.reduce()
    }

    pub fn multiply(&self, other: Fraction) -> Fraction {
        Fraction {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        }.reduce()
    }

    pub fn new(numerator: u32, denominator: u32) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn one() -> Fraction {
        Fraction {
            numerator: 1,
            denominator: 1
        }
    }

    pub fn reduce(&self) -> Fraction {
        let gcd = gcd(self.numerator, self.denominator);
        Fraction {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Sign {
    Negative,
    Positive,
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

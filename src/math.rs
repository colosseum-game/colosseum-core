use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Fraction(pub u32, pub u32);

impl Fraction {
    pub fn add(&self, other: Fraction) -> Fraction {
        Fraction(
            self.0 * other.1 + other.0 * self.1,
            self.1 * other.1
        ).reduce()
    }

    pub fn multiply(&self, other: Fraction) -> Fraction {
        Fraction(
            self.0 * other.0,
            self.1 * other.1,
        ).reduce()
    }

    pub fn one() -> Fraction {
        Fraction(1, 1)
    }

    pub fn reduce(&self) -> Fraction {
        let gcd = gcd(self.0, self.1);
        
        Fraction(
            self.0 / gcd,
            self.1 / gcd
        )
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

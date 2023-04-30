use std::fmt::{Display, Formatter, self};

#[derive(Debug, PartialEq, Clone)]
pub struct FiniteFieldElement {
    value: i32,
    modulus: i32,
}

impl FiniteFieldElement {
    pub fn new(value: i32, modulus: i32) -> FiniteFieldElement {
        FiniteFieldElement {
            value: value % modulus,
            modulus: modulus,
        }
    }

    fn check_zero(&self, b: &FiniteFieldElement) -> Result<(), String> {
        if b.value == 0 {
            Err("Zero division".to_string())
        } else {
            Ok(())
        }
    }

    fn check_modulus(&self, b: &FiniteFieldElement) -> Result<(), String> {
        if self.modulus != b.modulus {
            Err("Modulus mismatch".to_string())
        } else {
            Ok(())
        }
    }

    pub fn add(&self, b: &FiniteFieldElement) -> Result<FiniteFieldElement, String> {
        self.check_modulus(b)?; // check if modulus is the same
        Ok(FiniteFieldElement {
            value: (self.value + b.value).rem_euclid(self.modulus),
            modulus: self.modulus,
        })
    }

    pub fn sub(&self, b: &FiniteFieldElement) -> Result<FiniteFieldElement, String> {
        self.check_modulus(b)?;
        Ok(FiniteFieldElement {
            value: (self.value - b.value).rem_euclid(self.modulus),
            modulus: self.modulus,
        })
    }

    pub fn mul(&self, b: &FiniteFieldElement) -> Result<FiniteFieldElement, String> {
        self.check_modulus(b)?;
        Ok(FiniteFieldElement {
            value: (self.value * b.value).rem_euclid(self.modulus),
            modulus: self.modulus,
        })
    }

    pub fn div(&self, b: &FiniteFieldElement) -> Result<FiniteFieldElement, String> {
        self.check_modulus(b)?;
        self.check_zero(b)?;
        let (gcd, x, _) = ExtendedEuclideanAlgorithm::extended_gcd(b.value, self.modulus);
        println!("gcd: {}, x: {}", gcd, x);
        if gcd != 1 {
            return Err(format!("{} and {} are not coprimes", self.value, self.modulus));
        }
        Ok(FiniteFieldElement {
            value: (self.value * x).rem_euclid(self.modulus),
            modulus: self.modulus,
        })
    }

    pub fn pow(&self, b: u32) -> Result<FiniteFieldElement, String> {
        Ok(FiniteFieldElement {
            value: self.value.pow(b).rem_euclid(self.modulus),
            modulus: self.modulus,
        })
    }
}

impl Display for FiniteFieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(value: {},modulus: {})", self.value, self.modulus)
    }
}

// struct to calculate the gcd of two numbers using the extended euclidean algorithm
struct ExtendedEuclideanAlgorithm;


impl ExtendedEuclideanAlgorithm {
    // function to calculate gcd and bezout coefficients
    // algortihtm:
    // input: a, b integers from an euclidian domain
    // output: gcd(a, b), x, y such that ax + by = gcd(a, b) 
    // 1. r0 <- a, r1 <- b, x0 <- 1, x1 <- 0, y0 <- 0, y1 <- 1
    // 2. i <- 1
    // 3. while r_i != 0 do
    // 4.   divide r_i-1 by r_i to get quotient q_i and remainder r_i+1
    // 5.   x_i+1 <= x_i-1 - q_i * x_i
    // 6.   y_i+1 <= y_i-1 - q_i * y_i
    // 7.   i <- i + 1
    // 8. return r_i-1, x_i-1, y_i-1
    fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
        if b == 0 { // when remainder is 0, we have found the gcd
            return (a, 1, 0);
        }
        let (d, s, t) = ExtendedEuclideanAlgorithm::extended_gcd(b, a % b);
        (d, t , s - (a / b) * t)
    }
}


// tests for the finite field element operations
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = FiniteFieldElement::new(5, 7);
        let b = FiniteFieldElement::new(3, 7);
        let c = FiniteFieldElement::new(6, 7);

        assert_eq!(a.add(&b).unwrap(), FiniteFieldElement::new(1, 7));
        assert_eq!(b.add(&a).unwrap(), FiniteFieldElement::new(1, 7));
        assert_eq!(a.add(&c).unwrap(), FiniteFieldElement::new(4, 7));
    }

    #[test]
    fn test_sub() {
        let a = FiniteFieldElement::new(5, 7);
        let b = FiniteFieldElement::new(3, 7);
        let c = FiniteFieldElement::new(6, 7);

        assert_eq!(a.sub(&b).unwrap(), FiniteFieldElement::new(2, 7));
        assert_eq!(b.sub(&a).unwrap(), FiniteFieldElement::new(5, 7));
        assert_eq!(a.sub(&c).unwrap(), FiniteFieldElement::new(6, 7));
    }

    #[test]
    fn test_mul() {
        let a = FiniteFieldElement::new(5, 7);
        let b = FiniteFieldElement::new(3, 7);
        let c = FiniteFieldElement::new(6, 7);

        assert_eq!(a.mul(&b).unwrap(), FiniteFieldElement::new(1, 7));
        assert_eq!(b.mul(&a).unwrap(), FiniteFieldElement::new(1, 7));
        assert_eq!(a.mul(&c).unwrap(), FiniteFieldElement::new(2, 7));
    }

    #[test]
    fn test_div() {
        let a = FiniteFieldElement::new(5, 7);
        let b = FiniteFieldElement::new(3, 7);
        let c = FiniteFieldElement::new(4, 7);

        assert_eq!(a.div(&b).unwrap(), FiniteFieldElement::new(4, 7));
        assert_eq!(b.div(&a).unwrap(), FiniteFieldElement::new(2, 7));
        assert_eq!(a.div(&c).unwrap(), FiniteFieldElement::new(3, 7));
    }

    #[test]
    fn test_div_zero() {
        let a = FiniteFieldElement::new(5, 7);
        let b = FiniteFieldElement::new(0, 7);

        assert_eq!(a.div(&b).unwrap_err(), "Zero division");
    }

    #[test]
    fn test_div_not_coprime() {
        let a = FiniteFieldElement::new(1, 7);
        let b = FiniteFieldElement::new(-1, 7);

        assert_eq!(a.div(&b).unwrap_err(), "1 and 7 are not coprimes");
    }

    #[test]
    fn test_check_modulus() {
        let a = FiniteFieldElement::new(5, 7);
        let b = FiniteFieldElement::new(3, 7);
        let c = FiniteFieldElement::new(6, 8);

        assert_eq!(a.check_modulus(&b).unwrap(), ());
        assert_eq!(a.check_modulus(&c).unwrap_err(), "Modulus mismatch");
    }
}
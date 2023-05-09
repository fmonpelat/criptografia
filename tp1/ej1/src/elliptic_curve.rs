use std::{fmt::{Display, Formatter, self}};
use crate::finite_field_element::FiniteFieldElement;

#[derive(Debug, PartialEq, Clone)]


pub struct EllipticCurve {
    pub a: f64,
    pub b: f64,
}

impl EllipticCurve {
    pub fn new(a: f64, b: f64) -> EllipticCurve {
        EllipticCurve {
            a,
            b,
        }
    }

    pub fn eq(&self, other: &EllipticCurve) -> bool {
        self.a == other.a && self.b == other.b
    }

    pub fn check_point(&self, x: FiniteFieldElement, y: FiniteFieldElement) -> bool {
        let first = x.pow(3).expect("Error in check_point first argument");
        let second = x.mul(&FiniteFieldElement::new(self.a as i32, x.modulus)).expect("Error in check_point second argument");
        let third = FiniteFieldElement::new(self.b as i32, x.modulus);
        let y_squared = first.add(&second).expect("Error in check_point first argument").add(&third);
        y_squared == y.pow(2)
    }
}

impl Display for EllipticCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let a = if self.a < 0.0 {
            format!("- {}", self.a.abs())
        } else {
            format!("+ {}", self.a)
        };
        let b = if self.b < 0.0 {
            format!("- {}", self.b.abs())
        } else {
            format!("+ {}", self.b)
        };
        write!(f, "Elliptic Curve: y^2 = x^3 {}x {}", a, b)
    }
}

// testing module for Elliptic curve
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        assert_eq!(curve.a, -3.0);
        assert_eq!(curve.b, -3.0);
    }

    #[test]
    fn test_eq() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let other = EllipticCurve::new(-3.0, -3.0);
        assert_eq!(curve.eq(&other), true);
    }

    #[test]
    fn test_check_point() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = FiniteFieldElement::new(379, 1021);
        let y = FiniteFieldElement::new(1011, 1021);
        assert_eq!(curve.check_point(x, y), true);
    }


}


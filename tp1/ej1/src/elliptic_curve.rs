use crate::point::{Point, PointProperty};
use std::{fmt::{Display, Formatter, self}};

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

    pub fn calculate_abs(&self, x: PointProperty) -> Point {
        let y = (x.pow(3.0) + x.mul(self.a) + self.b).abs().sqrt();
        Point {
            x: Some(x),
            y: Some(y),
            curve: self.clone(),
        }
    }

    pub fn eq(&self, other: &EllipticCurve) -> bool {
        self.a == other.a && self.b == other.b
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
    fn test_calculate() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = 2.0;
        let point = curve.calculate_abs(x);
        assert_eq!(point.x.unwrap(), x);
        assert_eq!(point.y.unwrap(), 1.0);
        assert_eq!(curve.eq(&point.curve), true);
    }


}


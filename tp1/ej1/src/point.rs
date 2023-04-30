use crate::{elliptic_curve::EllipticCurve, finite_field_element::FiniteFieldElement, f64_element::F64element};
use std::fmt::{Display, Formatter, self};

#[derive(Debug, PartialEq, Clone)]

pub enum PointProperty {
    OptionF64(F64element),
    FiniteFieldElement(FiniteFieldElement),
}

impl PointProperty {
    pub fn mul(&self, b: &PointProperty) -> Result<f64, &str> {
        match self {
            PointProperty::OptionF64(f64_elem) => {
                Ok(f64_elem.mul(b).unwrap().value)
            },
            PointProperty::FiniteFieldElement(ff_elem) => ff_elem.mul(),
        }
    }
}

pub struct Point {
    pub x: Option<PointProperty>,
    pub y: Option<PointProperty>,
    pub curve: EllipticCurve,
}


impl Point {
    pub fn new(x: Option<PointProperty>, y: Option<PointProperty>, curve: EllipticCurve) -> Result<Point, String> {
        // inifinity denoted as x = None, y = None
        if x.is_none() && y.is_none() {
            return Ok(Point {
                x,
                y,
                curve,
            })
        }
        if x.is_none() || y.is_none() {
            return Err(format!("Point does not exist on curve: {}", curve).to_string())
        }
        let x = x.unwrap();
        let y = y.unwrap();
        // calculate the point on the curve if y < 0 then multiply by -1 after calculating because calculate only gives the positive y value  
        let curve_y = curve.calculate_abs(x).y.unwrap();
        let curve_y = if y < 0.0 { -curve_y } else { curve_y };
        // check that the point exists on the curve
        if curve_y != y {
            Err(format!("Point does not exist on curve: {} point: {} != {}", curve, curve_y, y).to_string())
        } else {
            Ok(Point {
                x: Some(x),
                y: Some(y),
                curve,
            })
        }      
    }

    pub fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.curve.eq(&other.curve)
    }

    pub fn sum(&self, other: &Point) -> Result<Point, String> {
        // checking that the points are on the same curve
        if !self.curve.eq(&other.curve) {
            return Err(format!("Points are not on the same curve: {} and {}", self.curve, other.curve).to_string())
        }

        // if points are the same then calculate using the tangent line
        if self.eq(&other) {
            // check that y is not 0 (tangent is a vertical line)
            if self.y.unwrap() == 0.0 {
                return Ok(Point {
                    x: None,
                    y: None,
                    curve: self.curve.clone(),
                })
            }
            // calculate the slope of the tangent line
            let slope = (3.0 * self.x.unwrap().powf(2.0) + self.curve.a) / (2.0 * self.y.unwrap());
            // calculate the x and y values of the new point
            // X3 = slope^2 - 2 * X1
            let x = slope.powf(2.0) - 2.0 * self.x.unwrap();
            // Y3 = slope * (X1 - X3) - Y1
            let y = slope * (self.x.unwrap() - x) - self.y.unwrap();
            return Ok( Point {
                x: Some(x),
                y: Some(y),
                curve: self.curve.clone()
            })
        }

        // return the other point if one of the points is infinity A + inf = A
        if self.x.is_none() && self.y.is_none() {
            return Ok(other.clone())
        }

        if other.x.is_none() && other.y.is_none() {
            return Ok(self.clone())
        }

        // check that the points are not inverses otherwise infinity is returned
        if self.x == other.x && self.y.unwrap() == -(other.y.unwrap()) {
            return Ok(Point {
                x: None,
                y: None,
                curve: self.curve.clone(),
            })
        }

        return Ok(Point {
            x: None,
            y: None,
            curve: self.curve.clone(),
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.x == None && self.y == None {
            return write!(f, "Point: Infinity")
        }
        write!(f, "Point: ({}, {})", self.x.unwrap(), self.y.unwrap())
    }
}

// testing module for Elliptic curve
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_point_exists() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = 2.0;
        let y = curve.calculate_abs(x).y.unwrap();
        let point = Point::new(Some(x), Some(y), curve);
        assert_eq!(point.is_ok(), true);
    }

    #[test]
    fn test_new_point_doesnt_exists() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let point2: Result<Point, String> = Point::new(Some(2.0), Some(2.0), curve);
        assert_eq!(point2.is_err(), true);
    }

    #[test]
    fn test_new_point_infinity() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let point3 = Point::new(None, None, curve);
        assert_eq!(point3.is_ok(), true);
        // check that x and y is None
        assert_eq!(point3.clone().unwrap().x, None);
        assert_eq!(point3.unwrap().y, None);
    }

    #[test]
    fn test_eq() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let point = Point::new(Some(2.0), Some(-1.0), curve.clone()).unwrap();
        let point2 = Point::new(Some(2.0), Some(-1.0), curve.clone()).unwrap();
        assert_eq!(point.eq(&point2), true);
    }

    #[test]
    fn test_sum_not_same_curve() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let curve2 = EllipticCurve::new(-4.0, -4.0);
        let x = 2.0;
        let y_curve1 =  curve.calculate_abs(x).y.unwrap();
        let y_curve2 = curve2.calculate_abs(x).y.unwrap();
        let point = Point::new(Some(x), Some(y_curve1), curve.clone()).unwrap();
        let point2 = Point::new(Some(x), Some(y_curve2), curve2.clone()).unwrap();
        let sum = point.sum(&point2);
        assert_eq!(sum.is_err(), true);
    }

    #[test]
    fn test_sum_same_points() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = 2.0;
        let y = curve.calculate_abs(x).y.unwrap();
        let point = Point::new(Some(x), Some(y), curve.clone()).unwrap();
        let point2 = Point::new(Some(x), Some(y), curve.clone()).unwrap();
        let sum = point.sum(&point2);
        assert_eq!(sum.is_ok(), true);
        let x_3 = 16.25;
        let y_3 = -65.125;
        assert_eq!(sum.clone().unwrap().x, Some(x_3));
        assert_eq!(sum.unwrap().y, Some(y_3));
    }

    #[test]
    fn test_sum_tangent_vertical() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        println!("{:?}", curve);
        let x = 2.1038034027355365331649473328289281;
        let y = curve.calculate_abs(x).y.unwrap();
        assert_eq!(y, 0.0);
        let point = Point::new(Some(x), Some(y), curve.clone()).unwrap();
        let point2 = Point::new(Some(x), Some(y), curve.clone()).unwrap();
        let sum = point.sum(&point2);
        assert_eq!(sum.is_ok(), true);
        assert_eq!(sum.clone().unwrap(), Point::new(None, None, curve.clone()).unwrap());
    }

    #[test]
    fn test_sum_with_inf() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = 2.0;
        let y = curve.calculate_abs(x).y.unwrap();
        let point = Point::new(Some(x), Some(y), curve.clone()).unwrap();
        let point2 = Point::new(None, None, curve.clone()).unwrap(); // here point is Inf

        let sum = point.sum(&point2);
        assert_eq!(sum.is_ok(), true);
        assert_eq!(sum.clone().unwrap().x, Some(x));
        assert_eq!(sum.unwrap().y, Some(y));

        let sum2 = point2.sum(&point);
        assert_eq!(sum2.is_ok(), true);
        assert_eq!(sum2.clone().unwrap(), Point::new(Some(x), Some(y), curve.clone()).unwrap()); 
    }

    #[test]
    fn test_sum_inverses() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = 2.0;
        let y = curve.calculate_abs(x).y.unwrap();
        let point = Point::new(Some(x), Some(y), curve.clone()).unwrap();
        let point2 = Point::new(Some(x), Some(-y), curve.clone()).unwrap();

        let sum = point.sum(&point2);
        assert_eq!(sum.is_ok(), true);
        assert_eq!(sum.clone().unwrap(), Point::new(None, None, curve.clone()).unwrap());
    }
}
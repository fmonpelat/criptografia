use crate::{elliptic_curve::EllipticCurve, finite_field_element::FiniteFieldElement};
use std::fmt::{Display, Formatter, self};

#[derive(Debug, PartialEq, Clone)]


pub struct Point {
    pub x: Option<FiniteFieldElement>,
    pub y: Option<FiniteFieldElement>,
    pub curve: EllipticCurve,
}


impl Point {
    pub fn new(x: Option<FiniteFieldElement>, y: Option<FiniteFieldElement>, curve: EllipticCurve) -> Result<Point, String> {
        // inifinity denoted as x = None, y = None
        if x.is_none() && y.is_none() {
            return Ok(Point {
                x,
                y,
                curve,
            })
        }
        if x.is_none() || y.is_none() {
            return Err(format!("Point does not exist on: {}", curve).to_string())
        }
        let x = x.expect("Error in Point::new x argument");
        let y = y.expect("Error in Point::new y argument");
        
        // check that the point exists on the curve
        if !curve.check_point(x.clone(), y.clone()) {
            Err(format!("Point does not exist on: {}", curve).to_string())
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

    fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    pub fn add(&self, other: &Point) -> Result<Point, String> {
        // checking that the points are on the same curve
        if !self.curve.eq(&other.curve) {
            panic!("Points are not on the same curve")
        }
        // checking if one of the points is infinity
        if  self.is_infinity() {
            return Ok(other.clone());
        }
        if other.is_infinity(){
            return Ok(self.clone());
        }
        // check if the points are inverses
        if self.x == other.x && self.y != other.y {
            return Point::new(None, None, self.curve.clone());
        }
        // checking if the points are the same
        if self.eq(other) {
            let x = self.x.clone().expect("Error in Point::add x argument");
            let y = self.y.clone().expect("Error in Point::add y argument");
            let slope_1 = 
                x.clone().pow(2).expect("Error in Point::add x^2 argument")
                .mul(&FiniteFieldElement::new(3, x.modulus)).expect("Error in Point::add x^2 argument multiplied by 3")
                .add(&FiniteFieldElement::new(self.curve.a.clone() as i32, x.modulus)).expect("Error in Point::add x^2 argument when adding a");
            let slope_2 = y.clone().mul(&FiniteFieldElement::new(2, x.modulus)).expect("Error in Point::add 2y argument");
            let slope = slope_1.div(&slope_2).expect("Error in Point::add slope argument");
             
            let x3 = slope.clone().pow(2).expect("Error in Point::add slope^2 argument")
                .sub(
                    &x.clone().mul(&FiniteFieldElement::new(2, x.modulus)).expect("Error in Point::add 2x argument")
                ).expect("Error in Point::add slope^2 argument when subtracting 2");
            let y3 = slope.clone().mul(
                &x.clone().sub(&x3.clone()).expect("Error in Point::add x - x3 argument")
            ).expect("Error in Point::add slope * (x - x3) argument")
                .sub(&y.clone()).expect("Error in Point::add slope * (x - x3) - y argument");
            return Point::new(Some(x3), Some(y3), self.curve.clone());
        }

        // points are different
        let x1 = self.x.clone().expect("Error in Point::add x1 argument");
        let y1 = self.y.clone().expect("Error in Point::add y1 argument");
        let x2 = other.x.clone().expect("Error in Point::add x2 argument");
        let y2 = other.y.clone().expect("Error in Point::add y2 argument");

        // slope of the line between the points 
        // s = (y2 - y1) / (x2 - x1)
        let s = y2.clone().sub(
            &y1.clone()
        ).expect("Error in Point::add y2 - y1 argument")
            .div(
                &x2.clone().sub(&x1.clone()).expect("Error in Point::add x2 - x1 argument")
            ).expect("Error in Point::add (y2 - y1) / (x2 - x1) argument");
        // x3 = s^2 - (x1 + x2)
        let x3 = s.clone().pow(2).expect("Error in Point::add s^2 argument")
            .sub(
                &x1.clone().add(&x2.clone()).expect("Error in Point::add x1 + x2 argument")
            ).expect("Error in Point::add s^2 - (x1 + x2) argument");
        // y3 = s(x1 - x3) - y1
        let y3 = s.clone().mul(
            &x1.clone().sub(&x3.clone()).expect("Error in Point::add x1 - x3 argument")
        ).expect("Error in Point::add s * (x1 - x3) argument")
            .sub(&y1.clone()).expect("Error in Point::add s * (x1 - x3) - y1 argument");
        // returning the new point
        return Point::new(Some(x3), Some(y3), self.curve.clone());
    }

    pub fn scalar_mul(&self, scalar: u32) -> Result<Point, String> {
        // starting the point at infinity
        let mut product = Point::new(None, None, self.curve.clone())?;

        let mut i = 0;
        while i < scalar {
            product = product.add(self)?;
            i += 1;
        }
        Ok(product)
    }

    pub fn naive_factor(&self, other: Point) -> Result<Option<u128>, String> {
        let mut i: u128 = 1;
        let mut generator = self.clone();
        while generator != other {
            generator = generator.add(self).expect("Error in Point::naive_factor generator.add(self) argument");
            i += 1;
        }
        if !generator.eq(&other) {
            return Ok(None)
        }
        Ok(Some(i))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.x == None && self.y == None {
            return write!(f, "Point: Infinity")
        }
        let x = self.x.clone().expect("Error in Point::fmt x argument");
        let y = self.y.clone().expect("Error in Point::fmt y argument");
        write!(f, "Point: ({}, {})", x, y)
    }
}

// testing module for Elliptic curve
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_point_exists() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = FiniteFieldElement::new(379, 1021);
        let y = FiniteFieldElement::new(1011, 1021);
        let point = Point::new(Some(x), Some(y), curve);
        assert_eq!(point.is_ok(), true);
    }

    #[test]
    fn test_new_point_doesnt_exists() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = FiniteFieldElement::new(1, 17);
        let y = FiniteFieldElement::new(5, 17);
        let point2: Result<Point, String> = Point::new(Some(x), Some(y), curve);
        println!("{:?}", point2);
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
        let x = FiniteFieldElement::new(379, 1021);
        let y = FiniteFieldElement::new(1011, 1021);
        let point = Point::new(Some(x.clone()), Some(y.clone()), curve.clone()).unwrap();
        let point2 = Point::new(Some(x), Some(y), curve.clone()).unwrap();
        assert_eq!(point.eq(&point2), true);
    }

    #[test]
    fn test_add_same_point() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = FiniteFieldElement::new(379, 1021);
        let y = FiniteFieldElement::new(1011, 1021);
        let point = Point::new(Some(x.clone()), Some(y.clone()), curve.clone()).unwrap();
        let point2 = Point::new(Some(x), Some(y), curve.clone()).unwrap();
        let point3 = point.add(&point2).unwrap();
        assert_eq!(point3.x, Some(FiniteFieldElement::new(57, 1021)));
        assert_eq!(point3.y, Some(FiniteFieldElement::new(914, 1021)));
    }

    #[test]
    fn test_add_different_points() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = FiniteFieldElement::new(379, 1021);
        let y = FiniteFieldElement::new(1011, 1021);
        let point = Point::new(Some(x.clone()), Some(y.clone()), curve.clone()).unwrap();
        let x2 = FiniteFieldElement::new(57, 1021);
        let y2 = FiniteFieldElement::new(914, 1021);
        let point2 = Point::new(Some(x2), Some(y2), curve.clone()).unwrap();
        let point3 = point.add(&point2).unwrap();
        assert_eq!(point3.x, Some(FiniteFieldElement::new(103, 1021)));
        assert_eq!(point3.y, Some(FiniteFieldElement::new(239, 1021)));
    }

    #[test]
    fn test_scalar_mul() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = FiniteFieldElement::new(379, 1021);
        let y = FiniteFieldElement::new(1011, 1021);
        let point = Point::new(Some(x.clone()), Some(y.clone()), curve.clone()).unwrap();

        // scalar mult by 2
        let point2 = point.scalar_mul(2).unwrap();
        assert_eq!(point2.x, Some(FiniteFieldElement::new(57, 1021)));
        assert_eq!(point2.y, Some(FiniteFieldElement::new(914, 1021)));

        // scalar mult by 3
        let point3 = point.scalar_mul(3).unwrap();
        assert_eq!(point3.x, Some(FiniteFieldElement::new(103, 1021)));
        assert_eq!(point3.y, Some(FiniteFieldElement::new(239, 1021)));

        // starting at inf returns inf
        let point = Point::new(None, None, curve.clone()).unwrap();
        let point3 = point.scalar_mul(2).unwrap();
        assert_eq!(point3.eq(&point), true);
    }

    #[test]
    fn test_is_on_curve() {
        let curve = EllipticCurve::new(-3.0, -3.0);
        let x = FiniteFieldElement::new(379, 1021);
        let y = FiniteFieldElement::new(1011, 1021);
        let point = Point::new(Some(x.clone()), Some(y.clone()), curve.clone());
        assert_eq!(point.is_ok(), true);

        let x2 = FiniteFieldElement::new(56, 1021);
        let y2 = FiniteFieldElement::new(914, 1021);
        let point2 = Point::new(Some(x2), Some(y2), curve.clone());
        assert_eq!(point2.is_err(), true);
    }

    #[test]
    fn test_naive_factor() {
        let curve = EllipticCurve::new(905.0, 100.0);
        let generator = Point::new(
            Some(FiniteFieldElement::new(1006, 1021)),
            Some(FiniteFieldElement::new(416, 1021)),
             curve.clone()
        ).expect("Error creating generator point doesnt belong to curve");
        let target = Point::new(
            Some(FiniteFieldElement::new(612, 1021)), 
            Some(FiniteFieldElement::new(827, 1021)),
            curve.clone()
        ).expect("Error creating target point doesnt belong to curve");
        let res = generator.naive_factor(target).expect("Error factoring point");
        println!("res: {:?}", res);
    }

}
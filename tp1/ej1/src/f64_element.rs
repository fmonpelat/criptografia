// create a wrapper for f64 that implements Element

use std::fmt::{Display, Formatter, self};
#[derive(Debug, PartialEq, Clone)]

pub struct F64element {
    pub value: f64,
}

impl F64element {
    pub fn new(value: f64) -> F64element {
        F64element {
            value,
        }
    }

    fn check_zero(&self, b: &F64element) -> Result<(), String> {
        if b.value == 0.0 {
            Err("Zero division".to_string())
        } else {
            Ok(())
        }
    }

    pub fn add(&self, b: &F64element) -> Result<F64element, String> {
        Ok(F64element {
            value: self.value + b.value,
        })
    }

    pub fn sub(&self, b: &F64element) -> Result<F64element, String> {
        Ok(F64element {
            value: self.value - b.value,
        })
    }

    pub fn mul(&self, b: &F64element) -> Result<F64element, String> {
        Ok(F64element {
            value: self.value * b.value,
        })
    }

    pub fn div(&self, b: &F64element) -> Result<F64element, String> {
        Ok(F64element {
            value: self.value / b.value,
        })
    }

    pub fn pow(&self, b: &F64element) -> Result<F64element, String> {
        Ok(F64element {
            value: self.value.powf(b.value),
        })
    }
}

impl Display for F64element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(value: {})", self.value)
    }
}



use ordered_float::NotNan;
use crate::style::css_values::*;

#[derive(Clone, Debug)]
pub struct ExactLength{
   pub unit : Unit,
   pub value: NotNan<f64>,
}

#[derive(Clone, Debug)]
pub enum Unit {
    Px,
    Rem,
    Em,
    Cm,
}

pub fn px<T:Into<f64>>(val:T) -> ExactLength {
    ExactLength{
        value: NotNan::new(val.into()).unwrap(),
        unit: crate::style::measures::Unit::Px,
    }
}

pub fn rgba(r:f64,g:f64,b:f64,a:f64) -> CssColor {
    CssColor::Rgba(r,g,b,a)
}
  

impl std::fmt::Display for ExactLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.unit {
            Unit::Px => write!(f,"{}px", self.value),
            Unit::Rem => write!(f,"{}rem", self.value),
            Unit::Em => write!(f,"{}em", self.value),
            Unit::Cm => write!(f,"{}cm", self.value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Percent(f64);


impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}%", self.0)
    }
}

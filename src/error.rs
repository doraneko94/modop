use std::fmt;

use crate::traits::Integer;

pub type Result<T> = std::result::Result<T, ModopError<T>>;

#[derive(Debug)]
pub enum ModopError<T: Integer> {
    CannotCalculate { object: &'static str, modulo: T },
    DifferentModulos { op: &'static str },
    NotRelativelyPrime { remainder: T, modulo: T },
}

impl<T: Integer> fmt::Display for ModopError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModopError::CannotCalculate { object, modulo } => {
                write!(f, "Cannot calculate {} with modulo {}!", object, modulo)
            }
            ModopError::DifferentModulos { op } => {
                write!(f, "Cannot {} between congruences with different modulos!", op)
            }
            ModopError::NotRelativelyPrime { remainder, modulo } => {
                write!(f, "Remainder {} and modulo {} must be relatively prime!", remainder, modulo)
            }
        }
    }
}
use std::fmt;

use crate::traits::Integer;

pub type Result<T> = std::result::Result<T, ModopError<T>>;

#[derive(Debug)]
pub enum ModopError<T: Integer> {
    DifferentModulos { op: &'static str },
    NotNaturalNumber { n: T },
    NotRelativelyPrime { remainder: T, modulo: T },
}

impl<T: Integer> fmt::Display for ModopError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModopError::DifferentModulos { op } => {
                write!(f, "Cannot {} between congruences with different modulos!", op)
            }
            ModopError::NotNaturalNumber { n } => {
                write!(f, "{} is not natural number!", n)
            }
            ModopError::NotRelativelyPrime { remainder, modulo } => {
                write!(f, "Remainder {} and modulo {} must be relatively prime!", remainder, modulo)
            }
        }
    }
}
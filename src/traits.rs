use num_traits::PrimInt;
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, ShrAssign};
use std::fmt::Display;

pub trait Integer: PrimInt + Display 
                 + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign + ShrAssign {}
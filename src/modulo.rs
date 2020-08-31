use crate::error::ModopError;
use crate::gcd::modinv;
use crate::traits::*;

use std::fmt;
use std::ops::{ Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };

/// It's better to set modulo to be smaller than T::max_value().sqrt()
#[derive(Clone, Copy)]
pub struct ModInt<T: Integer> {
    pub remainder: T,
    pub modulo: T,
}

impl<T: Integer> fmt::Debug for ModInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (mod {})", self.remainder, self.modulo)
    }
}

impl<T: Integer> ModInt<T> {
    pub fn new(value: T, modulo: T) -> Self {
        if modulo == T::zero() {
            panic!("Zero cannot be modulo!");
        }
        let remainder = value % modulo;
        Self { remainder, modulo }
    }

    pub fn pow(&mut self, mut n: usize) -> T {
        let mut a = self.remainder;
        let mut ans = T::one();
        loop {
            if n & 1 == 1 {
                ans *= a;
                ans %= self.modulo;
            }
            n >>= 1;
            if n <= 0 {
                break;
            }
            a = (a * a) % self.modulo;
        }
        ans
    }

    pub fn pow_into(&mut self, n: usize) -> T {
        self.remainder = self.pow(n);
        self.remainder
    }

    pub fn pow_inplace(&mut self, n: usize) {
        self.remainder = self.pow(n);
    }
}

impl<T: Integer> Add for ModInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.modulo != rhs.modulo {
            panic!("Cannot add between congruences with different modulos!")
        }
        let modulo = self.modulo;
        let remainder = (self.remainder + rhs.remainder) % modulo;
        Self { remainder, modulo }
    }
}

impl<T: Integer> Mul for ModInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self.modulo != rhs.modulo {
            panic!("Cannot multiply between congruences with different modulos!")
        }
        let modulo = self.modulo;
        let remainder = (self.remainder * rhs.remainder) % modulo;
        Self { remainder, modulo }
    }
}

impl<T: Integer> Div for ModInt<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if self.modulo != rhs.modulo {
            panic!("{}", ModopError::DifferentModulos::<T> { op: "divide" } )
        }
        match modinv(rhs.remainder, rhs.modulo) {
            Ok(i) => {
                let modulo = self.modulo;
                let remainder = (self.remainder * i) % modulo;
                Self { remainder, modulo }
            }
            Err(err) => {
                panic!("Division error: {}", err)
            }
        }
    }
}

macro_rules! impl_core {
    ($t:ty) => {
        impl Integer for $t {}
    };
}

macro_rules! impl_signed {
    ($t:ty) => {
        impl_core!($t);
        impl Sub for ModInt<$t> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                if self.modulo != rhs.modulo {
                    panic!("Cannot subtract between congruences with different modulos!")
                }
                let modulo = self.modulo;
                let remainder = (self.remainder - rhs.remainder) % modulo;
                Self { remainder, modulo }
            }
        }

        impl SubAssign for ModInt<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl ModInt<$t> {
            pub fn remainder_pos(&self) -> $t {
                if self.remainder >= 0 {
                    self.remainder
                } else {
                    self.remainder + self.modulo.abs()
                }
            }

            pub fn remainder_pos_into(&mut self) -> $t {
                self.remainder_pos_inplace();
                self.remainder
            }

            pub fn remainder_pos_inplace(&mut self) {
                self.remainder = self.remainder_pos();
            }

            pub fn remainder_neg(&self) -> $t {
                if self.remainder <= 0 {
                    self.remainder
                } else {
                    self.remainder - self.modulo.abs()
                }
            }

            pub fn remainder_neg_into(&mut self) -> $t {
                self.remainder_neg_inplace();
                self.remainder
            }

            pub fn remainder_neg_inplace(&mut self) {
                self.remainder = self.remainder_neg();
            }
        }
    };
}

macro_rules! impl_unsigned {
    ($t:ty) => {
        impl_core!($t);
        impl Sub for ModInt<$t> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                if self.modulo != rhs.modulo {
                    panic!("Cannot subtract between congruences with different modulos!")
                }
                let modulo = self.modulo;
                let remainder = if self.remainder >= rhs.remainder {
                    self.remainder - rhs.remainder
                } else {
                    modulo - rhs.remainder + self.remainder
                };
                Self { remainder, modulo }
            }
        }

        impl SubAssign for ModInt<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
    };
}

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);
impl_signed!(isize);

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(u128);
impl_unsigned!(usize);

impl<T: Integer> AddAssign for ModInt<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Integer> MulAssign for ModInt<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Integer> DivAssign for ModInt<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
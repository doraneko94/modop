use crate::error::{Result, ModopError};
use crate::traits::Integer;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn extended_gcd<T: Integer>(a: T, b: T) -> (T, (T, T)) {
    if b == T::zero() {
        (a, (T::one(), T::zero()))
    } else {
        let (d, (x, y)) = extended_gcd(b, a % b);
        let t = a / b;
        (d, (y, x - t * y))
    }
}

pub fn modinv<T: Integer>(mut a: T, m: T) -> Result<T> {
    if gcd(a, m) != T::one() {
        Err(ModopError::NotRelativelyPrime { remainder: a, modulo: m })
    } else {
        let m_i128 = m.to_i128().unwrap();
        let (mut b, mut u, mut v) = (m, 1i128, 0i128);
        loop {
            if b == T::zero() {
                break;
            }
            let t = a / b;
            let b_next = a - b * t;
            a = b;
            b = b_next;
            let v_next = u - t.to_i128().unwrap() * v;
            u = v;
            v = v_next;
        }
        u %= m_i128;
        if u < 0 {
            u += m_i128.abs();
        }
        Ok(T::from(u).unwrap())
    }
}
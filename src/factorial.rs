use crate::error::ModopError;
use crate::gcd::modinv;
use crate::modulo::ModInt;
use crate::traits::Integer;

pub struct ModGen<T: Integer> {
    pub factorials: Vec<T>,
    inv_factorials: Vec<Option<T>>,
    mod_max: ModInt<T>,
}

impl<T: Integer> ModGen<T> {
    pub fn new(modulo: T) -> Self {
        let one = T::one();
        if modulo == T::zero() {
            panic!("Zero cannot be modulo!");
        }
        let factorials = vec![one, one];
        let inv_factorial = match modinv(one, modulo) {
            Ok(i) => Some(i),
            Err(_) => None,
        };
        let inv_factorials = vec![inv_factorial, inv_factorial];
        let mod_max = ModInt::new(one, modulo);
        Self { factorials, inv_factorials, mod_max }
    }

    pub fn modulo(&self) -> T {
        self.mod_max.modulo
    }

    pub fn factorial(&mut self, n: usize) -> Result<ModInt<T>, ModopError<T>> {
        let modulo = self.modulo();
        self.expand(n);
        Ok(ModInt::new(self.factorials[n], modulo))
    }

    pub fn permutation(&mut self, n: usize, r: usize) -> Result<ModInt<T>, ModopError<T>> {
        let modulo = self.modulo();
        if n < r {
            panic!("Permutation error: n(={}) must be equal or larger than r(={})", n, r)
        }
        self.expand(n);
        let r_inv = self.inv_factorials[r];
        match r_inv {
            Some(i) => Ok(ModInt::new(self.factorials[n] * i, modulo)),
            None => Err(ModopError::CannotCalculate { object: "permutation", modulo: modulo })
        }
    }

    pub fn combination(&mut self, n: usize, r: usize) -> Result<ModInt<T>, ModopError<T>> {
        let modulo = self.modulo();
        if n < r {
            panic!("Combination error: n(={}) must be equal or larger than r(={})", n, r)
        }
        self.expand(n);
        let (r_inv, nr_inv) = (self.inv_factorials[r], self.inv_factorials[n-r]);
        if r_inv == None || nr_inv == None {
            Err(ModopError::CannotCalculate { object: "combination", modulo: modulo })
        } else {
            let comb = ModInt::new(self.factorials[n] * r_inv.unwrap(), modulo);
            Ok(ModInt::new(comb.remainder * nr_inv.unwrap(), modulo))
        }
    }

    pub fn expand(&mut self, n: usize) {
        let l = self.factorials.len();
        let modulo = self.modulo();
        if l <= n {
            for i in l..n+1 {
                self.mod_max *= ModInt::new(T::from(i).unwrap(), modulo);
                self.factorials.push(self.mod_max.remainder);
                match modinv(self.mod_max.remainder, modulo) {
                    Ok(j) => { self.inv_factorials.push(Some(j)); }
                    Err(_) => { self.inv_factorials.push(None); }
                };
            }
        }
    }
}
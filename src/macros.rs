#[macro_export]
macro_rules! set_mod {
    ($m:expr, $t:ty) => {
        macro_rules! modint {
            ($v:expr) => {
                ModInt::<$t>::new($v, $m)
            };
        }

        macro_rules! modgen {
            () => {
                ModGen::<$t>::new($m)
            };
        }
    };
}
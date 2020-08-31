use modop::set_mod;
use modop::modulo::*;

fn main() {
    set_mod!(1000000007, isize);
    let mut a = modint!(3);
    a.pow_inplace(45);
    println!("{:?}", a);
}
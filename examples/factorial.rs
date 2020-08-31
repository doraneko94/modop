use modop::*;

fn main() {
    set_mod!(7, usize);
    let mut g = modgen!();
    for i in 1..11 {
        println!("{:?}", g.permutation(i, 1));
    }
}
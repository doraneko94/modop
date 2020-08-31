use modop::*;

fn main() {
    set_mod!(5, isize);
    let a = modint!(3);
    // let c = ModInt::<isize>::new(3, 10);
    // let b = a / c; 'Cannot divide between congruences with different modulos!'
    // let b = a / modint!(5); 'Division error: Remainder 0 and modulo 5 must be relatively prime!'
    let mut b = a / modint!(4);
    println!("{:?}", b);
    b.remainder_neg_inplace();
    println!("{:?}", b);
    b.remainder_pos_inplace();
    println!("{:?}", b);
}
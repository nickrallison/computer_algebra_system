
mod sym;
use sym::*;

use num_complex::Complex64;


//TODO
//  minterm simplification
//      gather like terms
//  subs    
//  fixing braces
//  calculus
//      limits
//      chain rule
//      tree structure

/*macro_rules! matlarb {
    (syms $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}*/

fn main() {
    let complex_1: Complex64 = complex!(12, 1);
    let complex_2: Complex64 = complex!(12., -1.);
    let complex_sym_1: Sym = sym!(complex_1);
    let complex_sym_2: Sym = sym!(complex_2);
    //let one = symlit!(1.);
    //let two = symlit!(2.);
    //let x = symvar!("x");
    //let one_half = one.clone() / two.clone();
    //let root_two = two.clone() ^ one_half.clone();
    //let root_two_decimal = root_two.decimal();
    //let root_two_x = root_two.clone() / x.clone();
    println!("Here");
    println!("{}", (complex_sym_1))
}


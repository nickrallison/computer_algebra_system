
mod sym;
use sym::*;


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
    let complex_1: Sym = sym!(12., 1.);
    let complex_2: Sym = sym!(12., -1.);
    //let one = symlit!(1.);
    //let two = symlit!(2.);
    //let x = symvar!("x");
    //let one_half = one.clone() / two.clone();
    //let root_two = two.clone() ^ one_half.clone();
    //let root_two_decimal = root_two.decimal();
    //let root_two_x = root_two.clone() / x.clone();
    //println!("{} ", (root_two_x));
    //println!("{}", (complex_1 >= complex_2))
}


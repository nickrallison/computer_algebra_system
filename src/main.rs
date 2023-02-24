
use std::ops::{Add, Sub, Neg, Mul, Div, BitXor};
use std::{fmt, num};

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

macro_rules! symlit {
    ($ex: expr) => {
        Sym::Constant(Complex64{
            re: ($ex),
            im: (0 as f64),  
            }
        )
    };

    ($re: expr, $im: expr) => {
        Sym::Constant(Complex64{
            re: ($re),
            im: ($im),  
            }
        )
    };

}
macro_rules! symvar {
    ($ex: expr) => {
        Sym::Var(($ex).to_string())
    };
}

fn main() {
    let complex_1 = symlit!(12., 1.);
    let complex_2 = symlit!(12., -1.);
    let one = symlit!(1.);
    let two = symlit!(2.);
    let x = symvar!("x");
    let one_half = one.clone() / two.clone();
    let root_two = two.clone() ^ one_half.clone();
    //let root_two_decimal = root_two.decimal();
    let root_two_x = root_two.clone() / x.clone();
    println!("{} ", (complex_1*complex_2*x).decimal());
}


trait Symbolic {
    fn is_constant(&self) -> bool;
    fn decimal(&self) -> Sym;
    fn evaluate(&self) -> Sym;
}

trait Complex {
    fn is_imag(&self) -> bool;
    fn is_real(&self) -> bool;

    fn abs(&self) -> Complex64;
    fn angle(&self) -> Complex64;
}

#[derive(Clone, PartialEq)]
enum Func {
    Sum(Box<Sym>, Box<Sym>),
    Neg(Box<Sym>),
    Mult(Box<Sym>, Box<Sym>),
    Inv(Box<Sym>),
    Pow(Box<Sym>, Box<Sym>),
    Ln(Box<Sym>),
    Exp(Box<Sym>),
    Sin(Box<Sym>),
    SinInv(Box<Sym>),
    Cos(Box<Sym>),
    CosInv(Box<Sym>),
}

#[derive(Clone, PartialEq)]
enum Sym {
    Constant(Complex64),
    Var(String),
    Fun(Func)
}

impl Symbolic for Sym {

    fn is_constant(&self) -> bool {
        match self {
            Sym::Constant(_) => true,
            Sym::Var(_) => false,
            Sym::Fun(fun) => false
        }
    }

    fn evaluate(&self) -> Sym {
        match self {
            Sym::Constant(_) => return (*self).clone(),
            Sym::Var(_) => return (*self).clone(),
            Sym::Fun(fun) => {
                match *fun {
                    Func::Sum(_, _) => todo!(),
                    Func::Neg(_) => todo!(),
                    Func::Mult(_, _) => todo!(),
                    Func::Inv(_) => todo!(),
                    Func::Pow(_, _) => todo!(),
                    Func::Ln(_) => todo!(),
                    Func::Exp(_) => todo!(),
                    Func::Sin(_) => todo!(),
                    Func::SinInv(_) => todo!(),
                    Func::Cos(_) => todo!(),
                    Func::CosInv(_) => todo!(),
                }
            },
            
        }
    }

    fn decimal(&self) -> Sym {

        match self {
            Sym::Constant(_) => return (*self).clone(),
            Sym::Var(_) => return (*self).clone(),
            Sym::Fun(fun) => {
                match fun {
                    Func::Sum(a, b) => if (*a).is_constant() && (*b).is_constant() {
                        let mut a_val: Complex64; 
                        let mut b_val: Complex64;
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        match **b {
                            Sym::Constant(b_val_temp) => b_val = b_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val + b_val);
                    } else {a.decimal() + b.decimal()},
                    Func::Neg(a) =>  if (*a).is_constant(){
                        let mut a_val: Complex64; 
                        let mut b_val: Complex64;
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(-a_val);
                    } else {(-(**a).clone()).decimal()},
                    Func::Mult(a, b) =>  if (*a).is_constant() && (*b).is_constant() {
                        let mut a_val: Complex64; 
                        let mut b_val: Complex64;
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        match **b {
                            Sym::Constant(b_val_temp) => b_val = b_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val * b_val);
                    } else {a.decimal() * b.decimal()},
                    Func::Inv(a) => if (*a).is_constant(){
                        let mut a_val: Complex64; 
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(Complex64{
                            re: (1.),
                            im: (0.),   
                            } / a_val);
                    } else {symlit!(1.)/((*a).decimal())},
                    Func::Pow(a, b) => if (*a).is_constant() && (*b).is_constant() {
                        let mut a_val: Complex64; 
                        let mut b_val: Complex64;
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        match **b {
                            Sym::Constant(b_val_temp) => b_val = b_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val.powc(b_val));
                    } else {a.decimal() ^ b.decimal()},
                    Func::Ln(a) => if (*a).is_constant(){
                        let mut a_val: Complex64; 
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val.ln());
                    } else {Sym::Fun(Func::Ln(Box::new((*a).decimal())))},
                    Func::Exp(a) => if (*a).is_constant(){
                        let mut a_val: Complex64; 
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val.exp());
                    } else {Sym::Fun(Func::Exp(Box::new((*a).decimal())))},
                    Func::Sin(a) => if (*a).is_constant(){
                        let mut a_val: Complex64; 
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val.sin());
                    } else {Sym::Fun(Func::Sin(Box::new((*a).decimal())))},
                    Func::SinInv(a) => if (*a).is_constant(){
                        let mut a_val: Complex64; 
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val.asin());
                    } else {Sym::Fun(Func::SinInv(Box::new((*a).decimal())))},
                    Func::Cos(a) => if (*a).is_constant(){
                        let mut a_val: Complex64; 
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val.cos());
                    } else {Sym::Fun(Func::Cos(Box::new((*a).decimal())))},
                    Func::CosInv(a) => if (*a).is_constant(){
                        let mut a_val: Complex64; 
                        match **a {
                            Sym::Constant(a_val_temp) => a_val = a_val_temp,
                            Sym::Var(_) => todo!(),
                            Sym::Fun(_) => todo!(),
                        }
                        return Sym::Constant(a_val.acos());
                    } else {Sym::Fun(Func::CosInv(Box::new((*a).decimal())))},
                }
            },
            
        }
    }
}

impl Add for Sym {
    type Output = Self;

    fn add(self: Self, other: Self) -> Self::Output {
        Sym::Fun(Func::Sum(Box::new(self), Box::new(other)))
    }
}

impl Sub for Sym {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        (self + Sym::Fun(Func::Neg(Box::new(other))))
    }
}

impl Neg for Sym {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Sym::Fun(Func::Neg(Box::new(self)))
    }
}

impl Mul for Sym {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Sym::Fun(Func::Mult(Box::new(self), Box::new(other)))
    }
}

impl Div for Sym {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Sym::Fun(Func::Mult(Box::new(self), Box::new(Sym::Fun(Func::Inv(Box::new(other))))))
    }
}

impl BitXor for Sym {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        Sym::Fun(Func::Pow(Box::new(self), Box::new(other)))
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        let mut out: String = format_wrapper((*self).clone());
        write!(f, "{}", out)
    }
}

fn format_wrapper(sym: Sym) -> String {
    match sym {
        Sym::Constant(val) => return val.to_string(),

        Sym::Var(val) => return val,
        Sym::Fun(fun) => {
            let out = "".to_string();
            match fun {
                Func::Sum(a, b) => return out + "(" + &format_wrapper(*a) + " + " + &format_wrapper(*b) + ")",
                Func::Neg(a) => return out + "-" + &format_wrapper(*a),
                Func::Mult(a, b) => return out + "(" + &format_wrapper(*a) + " * " + &format_wrapper(*b) + ")",
                Func::Inv(a) => return out + "(1 / " + &format_wrapper(*a) + ")",
                Func::Pow(a, b) => return out + "(" + &format_wrapper(*a) + " ^ " + &format_wrapper(*b) + ")",
                Func::Ln(a) => return out + "ln(" + &format_wrapper(*a) + ")",
                Func::Exp(a) => return out + "exp(" + &format_wrapper(*a) + ")",
                Func::Sin(a) => return out + "sin(" + &format_wrapper(*a) + ")",
                Func::SinInv(a) => return out + "arcsin(" + &format_wrapper(*a) + ")",
                Func::Cos(a) => return out + "cos(" + &format_wrapper(*a) + ")",
                Func::CosInv(a) => return out + "arccos(" + &format_wrapper(*a) + ")",
            }
        },
    };
}
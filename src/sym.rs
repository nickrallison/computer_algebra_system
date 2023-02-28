
use std::ops::{Add, Sub, Neg, Mul, Div, BitXor};
use std::fmt;

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
// polynomial func

#[macro_export] macro_rules! symlit {
    ($ex: expr, &ty: expr) => {
        Sym::Constant(num_complex::Complex64{
            re: ($ex),
            im: (0 as f64),  
            }
        )
    };

    ($re: expr, $im: expr, &ty: expr) => {
        Sym::New(num_complex::Complex64{
            re: ($re),
            im: ($im),  
            }
        )
    };

}

#[macro_export] macro_rules! sym {
    ($ex: expr) => {
        expr.into();
    };

    ($real: expr, $imag: expr) => {
        num_complex::Complex64{
            re: ($real),
            im: ($imag),  
        }.into();
    };

}
#[macro_export] macro_rules! symvar {
    ($ex: expr) => {
        Sym::Var(($ex).to_string())
    };
}

#[derive(Clone, PartialEq)]
enum Func {
    //Sum(Box<Sym>, Box<Sym>),
    //Neg(Box<Sym>),
    //Mult(Box<Sym>, Box<Sym>),
    //Inv(Box<Sym>),
    //Pow(Box<Sym>, Box<Sym>),
    //Ln(Box<Sym>),
    //Exp(Box<Sym>),
    //Sin(Box<Sym>),
    //SinInv(Box<Sym>),
    //Cos(Box<Sym>),
    //CosInv(Box<Sym>),
}

#[derive(Clone, PartialEq)]
pub enum Sym {
    Constant(Complex64),
    Var(String),
    Fun,
    Minterm(Complex64, Vec<(Sym, Complex64)>),
    Polynomial(Vec<Sym>)
}

pub enum Equality {
    Equal(Box<Sym>, Box<Sym>)
    //NotEqual(Box<Sym>, Box<Sym>),
    //Greater(Box<Sym>, Box<Sym>),
    //GreaterEqual(Box<Sym>, Box<Sym>),
    //Less(Box<Sym>, Box<Sym>),
    //LessEqual(Box<Sym>, Box<Sym>)
}

pub trait Symbolic {
    fn New<T>(var: T) {

    }
}

pub trait SymbolicHelper {
    fn into(&self) -> Sym;
}

impl SymbolicHelper for &str {
    fn into(&self) -> Sym {
        Sym::Minterm(
            Complex64{
                re: 1.,
                im: 0.,
            }, 
            vec![(
                Sym::Var((*self).to_string()),  
                Complex64{
                    re: 1.,
                    im: 0.,
                }
            )])
    }
}

impl SymbolicHelper for String {
    fn into(&self) -> Sym {
        Sym::Minterm(
            Complex64{
                re: 1.,
                im: 0.,
            }, 
            vec![(
                Sym::Var(*self),  
                Complex64{
                    re: 1.,
                    im: 0.,
                }
            )])
    }
}

impl SymbolicHelper for i64 {
    fn into(&self) -> Sym {
        Sym::Minterm(
            Complex64{
                re: (*self as f64),
                im: 0.,
            }, 
            Vec::new()
        )
    }
}
impl SymbolicHelper for u64 {
    fn into(&self) -> Sym {
        Sym::Minterm(
            Complex64{
                re: (*self as f64),
                im: 0.,
            }, 
            Vec::new()
        )
    }
}
impl SymbolicHelper for f64 {
    fn into(&self) -> Sym {
        Sym::Minterm(
            Complex64{
                re: (*self),
                im: 0.,
            }, 
            Vec::new()
        )
    }
}
impl SymbolicHelper for Complex64 {
    fn into(&self) -> Sym {
        Sym::Minterm(
            *self, 
            Vec::new()
        )
    }
}

impl Symbolic for Sym {
    
}

impl Add for Sym {
    type Output = Self;

    fn add(self: Self, other: Self) -> Self::Output {
        
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
        Sym::Constant(val) => {
            if val.re == 0.0 && val.im == 0.0 {
                return "0".to_string();
            }
            else if val.re != 0.0 && val.im == 0.0 {
                return val.re.to_string();
            }
            else if val.re == 0.0 && val.im != 0.0 {
                return "".to_string() + &val.im.to_string()+"j";
            }
            return "".to_string() + "(" + &val.to_string() + ")";
        }


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
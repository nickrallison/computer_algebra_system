
use std::ops::{Add, Sub, Neg, Mul, Div, BitXor, Deref};
use std::fmt;

use decimal::d128;

//TODO
//  minterm simplification
//      gather like terms
//  subs    
//  fixing braces

fn main() {
    let one = Sym::Constant(d128!(1));
    let two = Sym::Constant(d128!(2));
    let x = Sym::Var("x".to_string());
    let one_half = one.clone() / two.clone();
    let root_two = two.clone() ^ one_half.clone();
    let root_two_decimal = root_two.decimal();
    let root_two_x = root_two / x;
    println!("{}", root_two_x.decimal());
}


trait Symbolic {
    fn eval(&self) -> Sym {
        return Sym::Constant(d128!(0));
    }

    fn decimal(&self) -> Sym {
        return Sym::Constant(d128!(0));
    }

    fn subs(&self, val: Sym) -> Sym {
        return Sym::Constant(d128!(0));
    }

}

#[derive(Clone, PartialEq)]
enum Sym {
    Constant(d128),
    Var(String),
    Neg(Box<Sym>),
    Sum((Box<Sym>, Box<Sym>)),
    Mult((Box<Sym>, Box<Sym>)),
    Div((Box<Sym>, Box<Sym>)),
    Exp((Box<Sym>, Box<Sym>))
}

impl Symbolic for Sym {

    fn eval(&self) -> Sym {
        match self {
            Sym::Constant(_) => return (*self).clone(),
            Sym::Var(_) => return (*self).clone(),
            Sym::Sum((sym_left, sym_right)) => return (*(*sym_left)).eval() + (*(*sym_right)).eval(),
            Sym::Mult((sym_left, sym_right)) => return (*(*sym_left)).eval() * (*(*sym_right)).eval(),
            Sym::Div((sym_left, sym_right)) => return (*(*sym_left)).eval() - (*(*sym_right)).eval(),
            Sym::Exp((sym_left, sym_right)) => (*(*sym_left)).eval() ^ (*(*sym_right)).eval(),
            Sym::Neg(_) => return -(*self).clone(),
        }
    }

    fn decimal(&self) -> Sym {

        match self {
            Sym::Constant(_) => return (*self).clone(),
            Sym::Var(_) => return (*self).clone(),
            Sym::Sum((sym_left, sym_right)) => {
                let sym_left_dec = sym_left.decimal(); 
                let sym_right_dec = sym_right.decimal();
                if matches!(sym_left_dec, Sym::Constant(_)) && matches!(sym_right_dec, Sym::Constant(_)) {
                    let mut val_left: d128 = d128!(0);
                    let mut val_right: d128 = d128!(0);
                    match sym_left_dec {
                        Sym::Constant(val_left_temp) => val_left = val_left_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }
                    match sym_right_dec {
                        Sym::Constant(val_right_temp) => val_right = val_right_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }
                    return Sym::Constant(val_left + val_right);
                } else {
                    return sym_left_dec + sym_right_dec;
                }
            },
            Sym::Mult((sym_left, sym_right)) => {
                let sym_left_dec = sym_left.decimal(); 
                let sym_right_dec = sym_right.decimal();
                if matches!(sym_left_dec, Sym::Constant(_)) && matches!(sym_right_dec, Sym::Constant(_)) {
                    let mut val_left: d128 = d128!(0);
                    let mut val_right: d128 = d128!(0);
                    match sym_left_dec {
                        Sym::Constant(val_left_temp) => val_left = val_left_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }
                    match sym_right_dec {
                        Sym::Constant(val_right_temp) => val_right = val_right_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }
                    return Sym::Constant(val_left * val_right);
                } else {
                    return sym_left_dec * sym_right_dec;
                }
            },
            Sym::Div((sym_left, sym_right)) => {
                let sym_left_dec = sym_left.decimal(); 
                let sym_right_dec = sym_right.decimal();
                if matches!(sym_left_dec, Sym::Constant(_)) && matches!(sym_right_dec, Sym::Constant(_)) {
                    let mut val_left: d128 = d128!(0);
                    let mut val_right: d128 = d128!(0);
                    match sym_left_dec {
                        Sym::Constant(val_left_temp) => val_left = val_left_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }
                    match sym_right_dec {
                        Sym::Constant(val_right_temp) => val_right = val_right_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }
                    return Sym::Constant(val_left / val_right);
                } else {
                    return sym_left_dec / sym_right_dec;
                }
            },
            Sym::Exp((sym_left, sym_right)) => {
                let sym_left_dec = sym_left.decimal(); 
                let sym_right_dec = sym_right.decimal();
                if matches!(sym_left_dec, Sym::Constant(_)) && matches!(sym_right_dec, Sym::Constant(_)) {
                    let mut val_left: d128 = d128!(0);
                    let mut val_right: d128 = d128!(0);
                    match sym_left_dec {
                        Sym::Constant(val_left_temp) => val_left = val_left_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }
                    match sym_right_dec {
                        Sym::Constant(val_right_temp) => val_right = val_right_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }
                    return Sym::Constant(val_left.pow(val_right));
                } else {
                    return sym_left_dec ^ sym_right_dec;
                }
            },
            Sym::Neg(sym) => {
                if matches!(**sym, Sym::Constant(_)) {
                    let mut val: d128 = d128!(0);
                    match **sym {
                        Sym::Constant(val_temp) => val = val_temp,
                        Sym::Var(_) => todo!(),
                        Sym::Neg(_) => todo!(),
                        Sym::Sum(_) => todo!(),
                        Sym::Mult(_) => todo!(),
                        Sym::Div(_) => todo!(),
                        Sym::Exp(_) => todo!(),
                    }

                    return Sym::Constant(-val);
                } else {
                    return Sym::Neg((*sym).clone());
                }
            }
        }
    }

    fn subs(&self, val: Sym) -> Sym {
        match *self {
            Sym::Constant(val) => return (*self).clone(),
            Sym::Var(_) => todo!(),
            Sym::Neg(_) => todo!(),
            Sym::Sum(_) => todo!(),
            Sym::Mult(_) => todo!(),
            Sym::Div(_) => todo!(),
            Sym::Exp(_) => todo!(),
        }
        return Sym::Constant(d128!(0));
    }
}

impl Add for Sym {
    type Output = Self;

    fn add(self: Self, other: Self) -> Self::Output {
        Sym::Sum((Box::new(self), Box::new(other)))
    }
}

impl Sub for Sym {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        (self + Sym::Neg(Box::new(other)))
    }
}

impl Neg for Sym {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Sym::Neg(Box::new(self))
    }
}

impl Mul for Sym {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Sym::Mult((Box::new(self), Box::new(other)))
    }
}

impl Div for Sym {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Sym::Div((Box::new(self), Box::new(other)))
    }
}

impl BitXor for Sym {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        Sym::Exp((Box::new(self), Box::new(other)))
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
        Sym::Neg(val) => {
            let mut out = "".to_string();
            return out + "-(" + &format_wrapper(*val) + ")";
        },
        Sym::Sum((val_left, val_right)) => {
            let mut out = "".to_string();
            return out + "(" + &format_wrapper(*val_left) + " + " + &format_wrapper(*val_right) + ")";
        },
        Sym::Mult((val_left, val_right)) => {
            let mut out = "".to_string();
            return out + "(" + &format_wrapper(*val_left) + " * " + &format_wrapper(*val_right) + ")";
        },
        Sym::Div((val_left, val_right)) => {
            let mut out = "".to_string();
            return out + "(" + &format_wrapper(*val_left) + " / " + &format_wrapper(*val_right) + ")";
        },
        Sym::Exp((val_left, val_right)) => {
            let mut out = "".to_string();
            return out + "(" + &format_wrapper(*val_left) + " ^ " + &format_wrapper(*val_right) + ")";
        },
    };
}
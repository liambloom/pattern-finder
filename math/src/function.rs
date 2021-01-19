

use core::ops::{Add, Sub, Mul, Div, Rem};
use alloc::{
    boxed::Box,
    fmt::Display,
};
use num::{rational::Ratio, traits::{NumOps, Pow}};
use nalgebra::ComplexField;
use paste::paste;
use Expression::*;

// Function is simply a wrapper class for Expression
pub struct Function<T: NumOps + Pow<T, Output = T>> {
    expr: Expression<T>
}

#[derive(Clone, Debug)]
enum Expression<T: NumOps + Pow<T, Output = T>> {
    Add(Box<Expression<T>>, Box<Expression<T>>),
    Sub(Box<Expression<T>>, Box<Expression<T>>),
    Mul(Box<Expression<T>>, Box<Expression<T>>),
    Div(Box<Expression<T>>, Box<Expression<T>>),
    Rem(Box<Expression<T>>, Box<Expression<T>>),
    Pow(Box<Expression<T>>, Box<Expression<T>>),
    Val(T),
    Independent,
}

impl<T: ComplexField + Pow<T, Output = T>> Function<T> {
    fn from_points(points: &[T]) -> Option<Self> {
        

        // Exponential
        if points.len() >= 3 {

        }

        None
    }
}

impl<T: NumOps + Pow<T, Output = T>> Expression<T> {
    fn is_const(&self) -> bool {
        match self {
            Add(a, b) 
            | Sub(a, b) 
            | Mul(a, b) 
            | Div(a, b) 
            | Rem(a, b) 
            | Pow(a, b) => a.is_const() && b.is_const(),
            Val(_) => true,
            Independent => false,
        }
    }
}

macro_rules! impl_ops {
    ($($trait:ident)*) => {
        $(
            impl<T: NumOps + Pow<T, Output = T>> $trait<Self> for Expression<T> {
                type Output = Self;

                paste! {
                    fn [<$trait:snake>](self, o: Self) -> Self {
                        if self.is_const() && o.is_const() {
                            if let Val(a) = self {
                                if let Val(b) = o {
                                    Val($trait::[<$trait:snake>](a, b))
                                }
                                else { unreachable!() }
                            }
                            else { unreachable!() }
                        }
                        else {
                            $trait(Box::new(self), Box::new(o))  
                        }
                    }
                }
            }

            impl<T: NumOps + Pow<T, Output = T>> $trait<T> for Expression<T> {
                type Output = Self;

                paste! {
                    fn [<$trait:snake>](self, v: T) -> Self {
                        self + Val(v)
                    }
                }
            }
        )*
    }
}

impl_ops!(Add Sub Mul Div Rem Pow);

impl<T: NumOps + Pow<T, Output = T>> From<T> for Expression<T> {
    fn from(val: T) -> Self {
        Val(val)
    }
}
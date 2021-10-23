use core::ops::{Add, Sub, Mul, Div, Bound};
use alloc::{
    boxed::Box,
    fmt::Display,
    vec::Vec,
};
use num::{rational::Ratio, traits::{NumOps, Pow}};
use nalgebra::ComplexField;
use paste::paste;
use Expression::*;
use inheriting_wrapper::use_inner;

// Function is simply a wrapper class for Expression
pub struct Function<T: NumOps + Pow<T, Output = T>> {
    expr: Expression<T>
}

impl<T: NumOps + Pow<T, Output = T>> Function<T> {

}

impl<T: ComplexField + Pow<T, Output = T>> Function<T> {
    fn from_points(points: &[T]) -> Option<Self> {
        

        // Exponential
        if points.len() >= 3 {

        }

        None
    }
}

#[derive(Clone, Debug)]
enum Expression<T: NumOps + Pow<T, Output = T>> {
    Add(Box<Expression<T>>, Box<Expression<T>>),
    Sub(Box<Expression<T>>, Box<Expression<T>>),
    Mul(Box<Expression<T>>, Box<Expression<T>>),
    Div(Box<Expression<T>>, Box<Expression<T>>),
    Pow(Box<Expression<T>>, Box<Expression<T>>),
    Val(T),
    Independent,
}

impl<T: NumOps + Pow<T, Output = T>> Function<T> {
    #[use_inner(expr: Expression<T>)]
    const FOO: i32;

    #[use_inner(expr)]
    pub fn is_const(&self) -> bool;

    #[use_inner(Expression::<T>)]
    fn bar() -> i32;
}

#[test]
fn use_inner_const_test() {
    assert_eq!(Function::<u16>::FOO, 2);
}

#[test]
fn use_inner_instance_fn_test() {
    for expr in [Independent, 2u32.into()].iter() {
        assert_eq!(expr.is_const(), Function { expr: expr.clone() }.is_const());
    }
}

#[test]
fn use_inner_static_fn_test() {
    assert_eq!(Function::<u16>::bar(), 5);
}

//#[impl_wrapper(Function<T: NumOps + Pow<T, Output = T>>)]
impl<T: NumOps + Pow<T, Output = T>> Expression<T> {
    const FOO: i32 = 2;

    fn is_const(&self) -> bool {
        match self {
            Add(a, b) 
            | Sub(a, b) 
            | Mul(a, b) 
            | Div(a, b) 
            | Pow(a, b) => a.is_const() && b.is_const(),
            Val(_) => true,
            Independent => false,
        }
    }

    fn bar() -> i32 {
        5
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

impl_ops!(Add Sub Mul Div Pow);

impl<T: NumOps + Pow<T, Output = T>> From<T> for Expression<T> {
    fn from(val: T) -> Self {
        Val(val)
    }
}

pub struct IntervalPart<T>(pub Bound<T>, pub Bound<T>);
pub type Interval<T> = Vec<IntervalPart<T>>;
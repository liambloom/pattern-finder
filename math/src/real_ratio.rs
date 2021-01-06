use std::{
    mem, fmt, num::ParseIntError, 
    ops::{Add, Sub, Mul, Div, Rem, Neg, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign}, 
    convert::{From, TryInto, Into}, 
};
use num::{Bounded, Num, One, Signed, Zero, pow::Pow, rational::Ratio, FromPrimitive};
use nalgebra::{ComplexField, Field, RealField};
use approx::{UlpsEq, RelativeEq, AbsDiffEq};
use simba::{simd::SimdValue, scalar::{SubsetOf, SupersetOf}};
use paste::paste;
use regex::Regex;

// Some of the trait implementations in this code are derived from the
// source code of the cates simba and approx

macro_rules! inherit {
    (static $($name:ident($($arg_name:ident: $arg_ty:ty),*)),* $(; $($next:tt)*)?) => {
        $(
            fn $name($($arg_name: $arg_ty),*) -> Self {
                RatioField::new(Ratio::$name($($arg_name),*))
            }
        )*
        $(inherit!($($next)*);)?
    };
    (static use i32 $($name:ident($($arg_name:ident: $arg_ty:ty),*)),* $(; $($next:tt)*)?) => {
        $(
            fn $name($($arg_name: $arg_ty),*) -> Self {
                RatioField::new(Ratio::from_integer(i32::$name($($arg_name),*)))
            }
        )*
        $(inherit!($($next)*);)?
    };
    (static use f64 $($name:ident($($arg_name:ident: $arg_ty:ty),*)),* $(; $($next:tt)*)?) => {
        $(
            fn $name($($arg_name: $arg_ty),*) -> Self {
                RatioField::new(Ratio::approximate_float(f64::$name($($arg_name),*)).unwrap())
            }
        )*
        $(inherit!($($next)*);)?
    };
    (use f64 $($name:ident($($arg_name:ident: $arg_ty:ty $(as $cast:ty)?),*)),* $(; $($next:tt)*)?) => {
        $(
            fn $name(self, $($arg_name: $arg_ty),*) -> Self {
                RatioField::new(Ratio::approximate_float(Into::<f64>::into(self).$name($($(Into::<$cast>::into)?($arg_name)),*)).unwrap())
            }
        )*
        $(inherit!($($next)*);)?
    };
    ($($name:ident($($arg_name:ident: Self),*)),* $(; $($next:tt)*)?) => {
        $(
            fn $name(self, $($arg_name: Self),*) -> Self {
                RatioField::new(self.ratio.$name($($arg_name.ratio),*))
            }
        )*
        $(inherit!($($next)*);)?
    };
    ($($name:ident($($arg_name:ident: &Self),*)),* -> $return:ty $(; $($next:tt)*)?) => {
        $(
            fn $name(&self, $($arg_name: Self),*) -> $return {
                self.ratio.$name($(&$arg_name.ratio),*)
            }
        )*
        $(inherit!($($next)*);)?
    };
    ($($name:ident($($arg_name:ident: &Self),*)),* $(; $($next:tt)*)?) => {
        $(
            fn $name(&self, $($arg_name: &Self),*) -> Self {
                RatioField::new(self.ratio.$name($(&$arg_name.ratio),*))
            }
        )*
        $(inherit!($($next)*);)?
    };
}

macro_rules! impl_op {
    (impl ($($trait:ident)*) for $struct:ty) => {
        $(
            impl $trait<$struct> for $struct {
                type Output = $struct;

                paste! {
                    inherit! {
                        [<$trait:snake>](o: Self)
                    }
                }
            }
            paste! {
                impl [<$trait Assign>] for $struct {
                    fn [<$trait:snake _assign>](&mut self, o: Self) {
                        self.ratio.[<$trait:snake _assign>](o.ratio)
                    }
                }
            }

            impl<'a> $trait for &'a $struct {
                type Output = $struct;

                paste! {
                    fn [<$trait:snake>](self, o: Self) -> $struct {
                        RatioField::new(self.ratio.[<$trait:lower>](o.ratio))
                    }
                }
            }
        )*
    };
}

macro_rules! impl_op_unary {
    ($($trait:ident)*) => {
        $(
            impl $trait for RatioField {
                //type Output = RatioField;

                paste! {
                    inherit! {
                        static [<$trait:lower>]();
                        [<is_ $trait:lower>]() -> bool
                    }
                }
            }
        )*
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct RatioField {
    pub ratio: Ratio<i32>
}

impl RatioField {
    const EPSILON: Self = Self::new(Ratio::new_raw(1, i32::MAX));

    pub const fn new(ratio: Ratio<i32>) -> Self {
        RatioField { ratio }
    }
}

impl From<Ratio<i32>> for RatioField {
    fn from(ratio: Ratio<i32>) -> Self {
        Self { ratio }
    }
}

impl Into<Ratio<i32>> for RatioField {
    fn into(self) -> Ratio<i32> {
        self.ratio
    }
}

impl_op!(impl (Add Sub Mul Div Rem) for RatioField);
impl_op_unary!(One Zero);

// Neg is unary, unlike the rest
impl Neg for RatioField {
    type Output = RatioField;

    inherit! {
        neg()
    }
}

impl Into<f64> for RatioField {
    fn into(self) -> f64 {
        let parts: (i32, i32) = self.ratio.into();
        parts.0 as f64 / parts.1 as f64
    }
}

impl RealField for RatioField {
    fn is_sign_positive(self) -> bool {
        self.is_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.is_negative()
    }

    fn copysign(self, to: Self) -> Self {
        if self.ratio.signum() == to.ratio.signum() {
            to
        }
        else {
            RatioField::new(-to.ratio)
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        if self.ratio > max.ratio {
            max
        }
        else if self.ratio < min.ratio {
            min
        }
        else {
            self
        }
    }

    fn atan2(self, other: Self) -> Self {
        RatioField::new(Ratio::approximate_float(Into::<f64>::into(self).atan2(other.into())).unwrap())
    }

    inherit! {
        max(other: Self), min(other: Self);
        static use f64 pi(), two_pi(), frac_pi_2(), frac_pi_3(), frac_pi_4(), frac_pi_6(), frac_pi_8(),
            frac_1_pi(), frac_2_pi(), frac_2_sqrt_pi(), e(), log2_e(), log10_e(), ln_2(), ln_10()
    }
}

impl Bounded for RatioField {
    inherit! {
        static use i32 min_value(), max_value()
    }
}

impl Signed for RatioField {
    inherit! {
        abs(), abs_sub(other: &Self), signum();
        is_positive(), is_negative() -> bool;
    }
}

impl Num for RatioField {
    type FromStrRadixErr = FromStrErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(FromStrErr::UnsupportedRadix)
        }
        let mut s = s.trim().to_owned();
        s.make_ascii_uppercase();
        if Regex::new(r"^-?[\dA-Z]*\.?\d+$").unwrap().is_match(&s) {
            Ok(RatioField::new(match s.find('.') {
                // The reason I don't just use Ratio::from_float(s.parse().unwrap()) is because of roundoff errors.
                // For example, Ratio::from_float(0.3).unwrap() = 5,404,319,552,844,595/18,014,398,509,481,984
                Some(mut index) => {
                    let negative = s.starts_with('-');
                    if negative {
                        s.remove(0);//(&s[1..s.len()]).to_string();
                        index -= 1;
                    }
                    let ratio = ( if index == 0 { num::zero() }
                        else { Ratio::from_integer((&s[0..index]).parse()?) } )
                    + Ratio::new(
                        (&s[index + 1..s.len()]).parse()?, 
                        10i32.pow((s.len() - index - 1).try_into().unwrap())
                    );
                    let neg = if negative { -1 } else { 1 };
                    ratio * neg
                },
                None => Ratio::from_integer(s.parse()?),
            }))
        }
        else if Regex::new(format!(r"^-?(?:\d+\s+)?\d+\s*/\s*-?\d+$").as_str()).unwrap().is_match(&s) {
            let number_regex =  Regex::new(r"-?\d+").unwrap();
            let mut matches: Vec<_> = number_regex.find_iter(&s).collect();
            let whole;
            if matches.len() == 3 {
                whole = Ratio::from_integer(matches.remove(0).as_str().parse()?);
            }
            else {
                whole = num::zero();
            }
            assert_eq!(matches.len(), 2);
            Ok(RatioField::new(
                whole + Ratio::new(if s.starts_with('-') { -1 } else { 1 } * num::abs(matches[0].as_str().parse::<i32>()?), 
                matches[1].as_str().parse()?)
            ))
        }
        else {
            match s.parse::<u32>() {
                Ok(_parsed) => Err(FromStrErr::InvalidNumber),
                Err(err) => Err(err.into()),
            }
        }
    }
}

impl ComplexField for RatioField {
    type RealField = RatioField;

    fn from_real(re: RatioField) -> Self {
        re
    }

    fn real(self) -> Self::RealField {
        self
    }

    fn imaginary(self) -> Self::RealField {
        Self::zero()
    }

    fn modulus(self) -> Self::RealField {
        Signed::abs(&self)
    }

    fn modulus_squared(self) -> Self::RealField {
        self * self
    }

    fn norm1(self) -> Self::RealField {
        Signed::abs(&self)
    }

    fn scale(self, factor: Self::RealField) -> Self {
        self * factor
    }

    fn unscale(self, factor: Self::RealField) -> Self {
        self / factor
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }

    fn hypot(self, o: Self) -> Self::RealField {
        (self.conjugate() * self + o.conjugate() * o).sqrt()
    }

    fn conjugate(self) -> Self {
        self
    }

    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    fn powi(self, n: i32) -> Self {
        Self::new(self.ratio.pow(n))
    }

    fn is_finite(&self) -> bool {
        true
    }

    fn try_sqrt(self) -> Option<Self> {
        Some(Self::new(Ratio::approximate_float(Into::<f64>::into(self).try_sqrt()?)?))
    }

    inherit! {
        floor(), ceil(), round(), trunc(), fract(), abs(), recip();
        use f64 sin(), cos(), tan(), asin(), acos(), atan(),
            sinh(), cosh(), tanh(), asinh(), acosh(), atanh(),
            log(base: Self::RealField as f64), log2(), log10(), ln(),
            ln_1p(), sqrt(), exp(), exp2(), exp_m1(), 
            powf(n: Self::RealField as f64), powc(n: Self as f64), cbrt(),
            argument()
    }
}

impl Field for RatioField { }

impl RelativeEq for RatioField {
    fn default_max_relative() -> Self {
        Self::EPSILON
    }

    fn relative_eq(&self, other: &Self, epsilon: Self, max_relative: Self) -> bool {
        if self == other {
            return true;
        }

        let abs_diff = (self - other).abs();

        // For when the numbers are really close together
        if abs_diff <= epsilon {
            return true;
        }

        let abs_self = self.abs();
        let abs_other = Signed::abs(other);

        let largest = if abs_other > abs_self {
            abs_other
        } else {
            abs_self
        };

        // Use a relative difference comparison
        abs_diff <= largest * max_relative
    }
}

impl UlpsEq for RatioField {
    fn default_max_ulps() -> u32 {
        4
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self, max_ulps: u32) -> bool {
        if Self::abs_diff_eq(self, other, epsilon) {
            return true;
        }

        if self.signum() != other.signum() {
            return false;
        }

        let int_self: i64 = unsafe { mem::transmute(Into::<f64>::into(*self)) };
        let int_other: i64 = unsafe { mem::transmute(Into::<f64>::into(*other)) };

        (int_self - int_other).abs() < max_ulps as i64
    }
}

impl SimdValue for RatioField {
    type Element = RatioField;
    type SimdBool = bool;

    fn lanes() -> usize {
        1
    }

    fn splat(val: Self::Element) -> Self {
        val
    }

    fn extract(&self, _: usize) -> Self::Element {
        *self
    }

    unsafe fn extract_unchecked(&self, _: usize) -> Self::Element {
        *self
    }

    fn replace(&mut self, _: usize, val: Self::Element) {
        *self = val
    }

    unsafe fn replace_unchecked(&mut self, _: usize, val: Self::Element) {
        *self = val
    }

    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        if cond {
            self
        } else {
            other
        }
    }
}

impl AbsDiffEq for RatioField {
    type Epsilon = Self;

    fn default_epsilon() -> Self {
        Self::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self) -> bool {
        (self - other).abs() <= epsilon
    }
}

impl fmt::Display for RatioField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ratio)
    }
}

impl FromPrimitive for RatioField {
    fn from_i64(n: i64) -> Option<Self> {
        match n.try_into() {
            Ok(n) => Some(Self::new(Ratio::from_integer(n))),
            Err(_) => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n.try_into() {
            Ok(n) => Some(Self::new(Ratio::from_integer(n))),
            Err(_) => None,
        }
    }
    
    fn from_f64(n: f64) -> Option<Self> {
        Some(Self::new(Ratio::approximate_float(n)?))
    }
}

impl SubsetOf<Self> for RatioField {
    fn to_superset(&self) -> Self {
        *self
    }

    fn from_superset_unchecked(element: &Self) -> Self {
        *element
    }

    fn is_in_subset(_: &Self) -> bool {
        true
    }
}

// Mathematically, this is not true. f64 represents real
// numbers, while Ratio represents rational numbers.
// ℝ ⊅ ℚ, but I have to pretend it does for this to
// work, so whatever
impl SupersetOf<f64> for RatioField {
    fn is_in_subset(&self) -> bool {
        true
    }

    fn to_subset_unchecked(&self) -> f64 {
        (*self).into()
    }

    fn from_subset(element: &f64) -> Self {
        RatioField::new(Ratio::approximate_float(*element).unwrap())
    }
}

pub enum FromStrErr {
    ParseIntError(ParseIntError),
    InvalidNumber,
    UnsupportedRadix
}

impl From<ParseIntError> for FromStrErr {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}
use core::{
    mem, fmt, num::ParseIntError, 
    ops::{Add, Sub, Mul, Div, Rem, Neg, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign}, 
    convert::{From, TryInto, Into}, 
};
use config::output::Output;
use num::{Bounded, FromPrimitive, Integer, Num, NumCast, One, Signed, Zero, pow::Pow, rational::Ratio, traits::NumAssign};
use nalgebra::{ComplexField, Field, RealField};
use approx::{UlpsEq, RelativeEq, AbsDiffEq};
use simba::{simd::SimdValue, scalar::{SubsetOf, SupersetOf}};
use paste::paste;
use alloc::borrow::ToOwned;

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
    (static use T $($name:ident($($arg_name:ident: $arg_ty:ty),*)),* $(; $($next:tt)*)?) => {
        $(
            fn $name($($arg_name: $arg_ty),*) -> Self {
                RatioField::new(Ratio::from_integer(T::$name($($arg_name),*)))
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
    ($($trait:ident)*) => {
        $(
            impl<T: Clone + Integer> $trait<RatioField<T>> for RatioField<T> {
                type Output = Self;

                paste! {
                    inherit! {
                        [<$trait:snake>](o: Self)
                    }
                }
            }
            paste! {
                impl<T: Clone + Integer + NumAssign> [<$trait Assign>] for RatioField<T> {
                    fn [<$trait:snake _assign>](&mut self, o: Self) {
                        self.ratio.[<$trait:snake _assign>](o.ratio)
                    }
                }
            }

            impl<'a, T: Clone + Integer + Copy> $trait for &'a RatioField<T> {
                type Output = RatioField<T>;

                paste! {
                    fn [<$trait:snake>](self, o: Self) -> Self::Output {
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
            impl<T: Clone + Integer> $trait for RatioField<T> {
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
pub struct RatioField<T: Integer + Clone> {
    pub ratio: Ratio<T>
}

impl<T: Clone + Integer> RatioField<T> {
    pub fn new(ratio: Ratio<T>) -> Self {
        RatioField { ratio }
    }
}

impl<T: Clone + Integer> From<Ratio<T>> for RatioField<T> {
    fn from(ratio: Ratio<T>) -> Self {
        Self { ratio }
    }
}

impl<T: Clone + Integer> Into<Ratio<T>> for RatioField<T> {
    fn into(self) -> Ratio<T> {
        self.ratio
    }
}

impl_op!(Add Sub Mul Div Rem);
impl_op_unary!(One Zero);

// Neg is unary, unlike the rest
impl<T: Clone + Integer + Neg<Output = T>> Neg for RatioField<T> {
    type Output = Self;

    inherit! {
        neg()
    }
}

impl<T: Integer + Clone> Into<f64> for RatioField<T> {
    fn into(self) -> f64 {
        let parts: (T, T) = self.ratio.into();
        to_f64(parts.0) / to_f64(parts.1)
    }
}

fn to_f64<T: Integer + Clone>(mut n: T) -> f64 {
    let mut digit = T::one();
    let mut digit_f = 1.0;
    let two = T::one() + T::one();
    while digit <= n {
        digit = digit * two.clone();
        digit_f *= 2.0;
    }
    digit = digit / two.clone();
    digit_f /= 2.0;
    let mut float = 0.0;
    while n > T::zero() {
        if n.is_multiple_of(&digit) {
            n = n - digit.clone();
            float += digit_f;
            debug_assert!(n < digit.clone());
        }
        digit = digit / two.clone();
        digit_f /= 2.0;
    }
    float
}

impl<T> RealField for RatioField<T>
    where T: Integer + Clone + Signed + Bounded + fmt::Debug + Copy + Sync + 
        NumAssign + Send + NumCast + fmt::Display + Pow<u32, Output = T> + 'static + FromPrimitive
{
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

impl<T: Integer + Clone + Bounded> Bounded for RatioField<T> {
    inherit! {
        static use T min_value(), max_value()
    }
}

impl<T: Integer + Clone + Signed> Signed for RatioField<T> {
    inherit! {
        abs(), abs_sub(other: &Self), signum();
        is_positive(), is_negative() -> bool;
    }
}

impl<T: Integer + Clone> Num for RatioField<T> {
    type FromStrRadixErr = FromStrErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(FromStrErr::UnsupportedRadix)
        }
        else {
            //Ok(RatioField::new(util::parse(s)?))
            todo!()
        }
    }
}

impl<T> ComplexField for RatioField<T>
    where T: Integer + Clone + Signed + Bounded + fmt::Debug + Copy + Sync + NumAssign + 
        Send + NumCast + fmt::Display + 'static + Pow<u32, Output = T> + FromPrimitive
{
    type RealField = Self;

    fn from_real(re: Self) -> Self {
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
        let parts: (T, T) = self.ratio.into();
        //RatioField::new(Ratio::new(parts.0.pow(n), parts.1.pow(n)))
        //todo!()
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

impl<T: Integer + Clone + Signed + NumAssign> Field for RatioField<T> { }

impl<T: Integer + Clone + Signed + Bounded> RelativeEq for RatioField<T> {
    fn default_max_relative() -> Self {
        Self::default_epsilon()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self, max_relative: Self) -> bool {
        if self == other {
            return true;
        }

        let abs_diff = (self.clone() - other.clone()).abs();

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

impl<T: Integer + Clone + Signed + Bounded> UlpsEq for RatioField<T> {
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

        let int_self: i64 = unsafe { mem::transmute(Into::<f64>::into(self.clone())) };
        let int_other: i64 = unsafe { mem::transmute(Into::<f64>::into(other.clone())) };

        (int_self - int_other).abs() < max_ulps as i64
    }
}

impl<T: Integer + Clone> SimdValue for RatioField<T> {
    type Element = Self;
    type SimdBool = bool;

    fn lanes() -> usize {
        1
    }

    fn splat(val: Self::Element) -> Self {
        val
    }

    fn extract(&self, _: usize) -> Self::Element {
        self.to_owned()
    }

    unsafe fn extract_unchecked(&self, _: usize) -> Self::Element {
        self.to_owned()
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

impl<T> AbsDiffEq for RatioField<T>
    where T: Clone + Integer + Bounded + Signed
{
    type Epsilon = Self;

    fn default_epsilon() -> Self::Epsilon {
        Self::new(Ratio::new(T::one(), T::max_value()))
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self) -> bool {
        (self.clone() - other.clone()).abs() <= epsilon
    }
}

impl<T: Integer + Clone + fmt::Display> fmt::Display for RatioField<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ratio)
    }
}

impl<T: Integer + Clone + FromPrimitive + Bounded + NumCast + Signed> FromPrimitive for RatioField<T> {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::new(Ratio::from_integer(T::from_i64(n)?)))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::new(Ratio::from_integer(T::from_u64(n)?)))
    }
    
    fn from_f64(n: f64) -> Option<Self> {
        Some(Self::new(Ratio::approximate_float(n)?))
    }
}

impl<T: Integer + Clone> SubsetOf<Self> for RatioField<T> {
    fn to_superset(&self) -> Self {
        self.to_owned()
    }

    fn from_superset_unchecked(element: &Self) -> Self {
        element.to_owned()
    }

    fn is_in_subset(_: &Self) -> bool {
        true
    }
}

// Mathematically, this is not true. f64 represents real
// numbers, while Ratio represents rational numbers.
// ℝ ⊅ ℚ, but I have to pretend it does for this to
// work, so whatever
impl<T: Integer + Clone + Bounded + NumCast + Signed> SupersetOf<f64> for RatioField<T> {
    fn is_in_subset(&self) -> bool {
        true
    }

    fn to_subset_unchecked(&self) -> f64 {
        self.to_owned().into()
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
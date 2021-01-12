use num::{Zero, rational::Ratio};
use config::fmt::{FmtAble, FmtEr};
use crate::{real_ratio::RatioField, util::all_equal};
use nalgebra::{DMatrix as Matrix, DVector as Vector};

#[derive(Debug)]
pub struct Polynomial {
    terms: Vec<Term>,
}
impl Polynomial {
    pub fn from_values(values: &Vec<Ratio<i32>>, starting: i32) -> Option<Self> {
        let mut points = Vec::with_capacity(values.len());
        for i in values.iter().enumerate() {
            points.push((Ratio::from_integer(i.0 as i32 + starting), *i.1));
        }
        Self::from_points(&points)
    }
    pub fn from_points(points: &Vec<(Ratio<i32>, Ratio<i32>)>) -> Option<Self> {
        let len = points.len();
        let range = (0..len as i32).rev();
        let mut x_matrix: Vec<RatioField> = Vec::with_capacity(len.pow(2));
        let mut y_vector: Vec<RatioField> = Vec::with_capacity(len);
        for point in points {
            for i in range.clone() {
                x_matrix.push(point.0.pow(i).into());
            }
            y_vector.push(point.1.into());
        }
        let mut terms: Vec<Term> = (Matrix::from_row_slice(len, len, &x_matrix).try_inverse()? * Vector::from_vec(y_vector))
            .data.as_vec().iter().enumerate().filter(|e| *e.1 != RatioField::zero())
            .map(|i| Term { coefficient: (*i.1).into(), exponent: (len - 1 - i.0) as u8 } )
            .collect();
        if terms.len() == 0 {
            terms = vec![ Term { coefficient: Ratio::zero(), exponent: 0} ]
        }
        Some(Self {
            terms
        })
    }
}
impl FmtAble for Polynomial {
    fn format(&self, f: &impl FmtEr) -> String {
        let mut iter = self.terms.iter();
        let mut s: String = iter.next().unwrap().format(f);
        for term in iter {
            s = f.add(s.as_str(), term.format(f).as_str());
        }
        if s.is_empty() {
            String::from("0")
        }
        else {
            s
        }
    }
}

#[derive(Debug)]
struct Term {
    pub coefficient: Ratio<i32>,
    pub exponent: u8,
}
impl Term {
    fn apply(&self, value: &i32) -> Ratio<i32> {
        self.apply_ratio(&Ratio::from_integer(*value))
    }
    fn apply_ratio(&self, x: &Ratio<i32>) -> Ratio<i32> {
        self.coefficient * x.pow(self.exponent as i32)
    }
}
    
impl FmtAble for Term {
    fn format(&self, f: &impl FmtEr) -> String {
        let mut s = String::new();
        if self.coefficient != num::zero() {
            let coeff_abs = num::abs(self.coefficient);
            if coeff_abs != num::one() || self.exponent == 0 {
                let mut coeff_str = f.divide(coeff_abs.numer().to_string().as_str(), coeff_abs.denom().to_string().as_str());
                if self.coefficient.denom() != &1 {
                    coeff_str = format!("({})", coeff_str);
                }
                s += coeff_str.as_str();
            }
            let x_exp = if self.exponent == 1 { f.x().to_string() }
                else if self.exponent > 1 { f.pow(f.x().to_string().as_str(), self.exponent.to_string().as_str()) }
                else { String::new() };
            if s.len() == 0 { s = x_exp }
            else if x_exp.len() != 0 { s = f.multiply(s.as_str(), x_exp.as_str())}
            if self.coefficient < num::zero() {
                s = f.neg(&s);
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;
    use config::fmt::{formatters, FmtAble};
    use num::rational::Ratio;
    use crate::util::as_ratios;
    const ASCII: formatters::ASCII = formatters::ASCII;

    mod horizontal {
        use super::*;

        #[test]
        fn zero() {
            let zero = num::zero();
            assert_eq!(Polynomial::from_values(&vec![zero, zero, zero], 0).unwrap().format(&ASCII), "0");
        }

        #[test]
        fn one() {
            let one = num::one();
            assert_eq!(Polynomial::from_values(&vec![one, one, one], 0).unwrap().format(&ASCII), "1");
        }
    }
    
    mod linear {
        use super::*;

        #[test]
        fn parent() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 1, 2]), 0).unwrap().format(&ASCII), "x");
        }

        #[test]
        fn translated() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![1, 2, 3]), 0).unwrap().format(&ASCII), "x+1");
        }

        #[test]
        fn stretched() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 2, 4]), 0).unwrap().format(&ASCII), "2x");
        }

        #[test]
        fn compressed() {
            assert_eq!(Polynomial::from_values(&vec![num::zero(), Ratio::new(1, 2), num::one()], 0).unwrap().format(&ASCII), "(1/2)x");
        }

        #[test]
        fn reflected() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, -1, -2]), 0).unwrap().format(&ASCII), "-x");
        }

        #[test]
        fn all() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![1, -1, -3]), 0).unwrap().format(&ASCII), "-2x+1");
        }
    }

    mod quadratic {
        use super::*;

        #[test]
        fn parent() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 1, 4, 9]), 0).unwrap().format(&ASCII), "x^2");
        }

        #[test]
        fn vertical_translated() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![1, 2, 5, 10]), 0).unwrap().format(&ASCII), "x^2+1");
        }

        #[test]
        fn horizontal_translated() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![1, 4, 9, 16]), 0).unwrap().format(&ASCII), "x^2+2x+1");
        }

        #[test]
        fn stretched() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 2, 8, 18]), 0).unwrap().format(&ASCII), "2x^2");
        }

        #[test]
        fn compressed() {
            assert_eq!(Polynomial::from_values(&vec![num::zero(), Ratio::new(1, 2), Ratio::from_integer(2), Ratio::new(9, 2)], 0).unwrap().format(&ASCII), "(1/2)x^2");
        }

        #[test]
        fn reflected() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, -1, -4, -9]), 0).unwrap().format(&ASCII), "-x^2");
        }

        #[test]
        fn all() {
            assert_eq!(Polynomial::from_values(&vec![Ratio::new(-1, 2), Ratio::from_integer(-2), Ratio::new(-9, 2), Ratio::from_integer(-8)], 0).unwrap().format(&ASCII), "-(1/2)x^2-x-(1/2)");
        }
    }

    #[test]
    fn many_intercepts() {
        assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 0, 0, 6, 24]), 0).unwrap().format(&ASCII), "x^3-3x^2+2x");
    }
}
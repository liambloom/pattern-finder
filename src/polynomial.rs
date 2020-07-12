use num::rational::Ratio;
use crate::fmt::{FmtEr, FmtAble};
use crate::util::all_equal;

#[derive(Debug)]
pub struct Polynomial {
    terms: Vec<Term>,
}
impl Polynomial {
    pub fn from_values(values: &Vec<Ratio<isize>>) -> Option<Self> {
        if values.len() < 2 {
            None
        }
        else if all_equal(values) {
            Some(Self {
                terms: vec![Term {
                    coefficient: values[0],
                    exponent: 0
                }]
            })
        }
        else {
            let mut p = Self {
                terms: vec![Term::from_values(values)?],
            };
            if p.terms[0].coefficient != num::zero() {
                let mut divided = Vec::new();
                for i in values.iter().enumerate() {
                    divided.push(i.1 - p.terms[0].apply(&(i.0 as isize)));
                }
                let mut sub = Self::from_values(&divided).unwrap_or_else(|| Self::EMPTY).terms;
                if sub.len() == 1 && sub[0].coefficient == num::zero() {
                    sub = Vec::new();
                }
                p.terms.append(&mut sub);
            }
            Some(p)
        }
    }
    const EMPTY: Self = Self { terms: Vec::new() };
}
impl FmtAble for Polynomial {
    fn format(&self, f: &impl FmtEr) -> String {
        println!("{:?}", &self.terms);
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
    pub coefficient: Ratio<isize>,
    pub exponent: u8,
}
impl Term {
    pub fn from_values(values: &Vec<Ratio<isize>>) -> Option<Self> {
        if values.len() < 3 {
            return None
        }
        let mut diffs: Vec<Ratio<isize>> = Vec::new();
        for i in 0..values.len() - 1 {
            diffs.push(values[i + 1] - values[i]);
        };
        if all_equal(&diffs) {
            Some(Term {
                coefficient: diffs[0],
                exponent: 1,
            })
        }
        else {
            let mut rec = Term::from_values(&diffs)?;
            rec.exponent += 1;
            rec.coefficient /= Ratio::from_integer(rec.exponent as isize);
            Some(rec)
        }
    }
    pub fn apply(&self, x: &isize) -> Ratio<isize> {
        self.apply_ratio(&Ratio::from_integer(*x))
    }
    pub fn apply_ratio(&self, x: &Ratio<isize>) -> Ratio<isize> {
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
    use super::{Polynomial};
    use crate::fmt::{formatters, FmtAble};
    use num::rational::Ratio;
    const ASCII: formatters::ASCII = formatters::ASCII;

    fn as_ratios(vec: Vec<isize>) -> Vec<Ratio<isize>> { // Not a test, just used by tests
        let mut new = Vec::new();
        for int in vec.iter() {
            new.push(Ratio::from_integer(*int));
        }
        new
    }

    mod horizontal {
        use super::*;

        #[test]
        fn zero() {
            let zero = num::zero();
            assert_eq!(Polynomial::from_values(&vec![zero, zero, zero]).unwrap().format(&ASCII), "0");
        }

        #[test]
        fn one() {
            let one = num::one();
            assert_eq!(Polynomial::from_values(&vec![one, one, one]).unwrap().format(&ASCII), "1");
        }
    }
    
    mod linear {
        use super::*;

        #[test]
        fn parent() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 1, 2])).unwrap().format(&ASCII), "x");
        }

        #[test]
        fn translated() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![1, 2, 3])).unwrap().format(&ASCII), "x+1");
        }

        #[test]
        fn stretched() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 2, 4])).unwrap().format(&ASCII), "2x");
        }

        #[test]
        fn compressed() {
            assert_eq!(Polynomial::from_values(&vec![num::zero(), Ratio::new(1, 2), num::one()]).unwrap().format(&ASCII), "(1/2)x");
        }

        #[test]
        fn reflected() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, -1, -2])).unwrap().format(&ASCII), "-x");
        }

        #[test]
        fn all() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![1, -1, -3])).unwrap().format(&ASCII), "-2x+1");
        }
    }

    mod quadratic {
        use super::*;

        #[test]
        fn parent() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 1, 4, 9])).unwrap().format(&ASCII), "x^2");
        }

        #[test]
        fn vertical_translated() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![1, 2, 5, 10])).unwrap().format(&ASCII), "x^2+1");
        }

        #[test]
        fn horizontal_translated() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![1, 4, 9, 16])).unwrap().format(&ASCII), "x^2+2x+1");
        }

        #[test]
        fn stretched() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 2, 8, 18])).unwrap().format(&ASCII), "2x^2");
        }

        #[test]
        fn compressed() {
            assert_eq!(Polynomial::from_values(&vec![num::zero(), Ratio::new(1, 2), Ratio::from_integer(2), Ratio::new(9, 2)]).unwrap().format(&ASCII), "(1/2)x^2");
        }

        #[test]
        fn reflected() {
            assert_eq!(Polynomial::from_values(&as_ratios(vec![0, -1, -4, -9])).unwrap().format(&ASCII), "-x^2");
        }

        #[test]
        fn all() {
            assert_eq!(Polynomial::from_values(&vec![Ratio::new(-1, 2), Ratio::from_integer(-2), Ratio::new(-9, 2), Ratio::from_integer(-8)]).unwrap().format(&ASCII), "-(1/2)x^2-x-(1/2)");
        }
    }

    #[test]
    fn many_intercepts() {
        assert_eq!(Polynomial::from_values(&as_ratios(vec![0, 0, 0, 6, 24])).unwrap().format(&ASCII), "x^3-3x^2+2x");
    }
}
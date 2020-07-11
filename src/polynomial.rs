use num::rational::Ratio;
use crate::util;
use crate::fmt::{FmtEr, FmtAble};

#[derive(Debug)]
pub struct Polynomial {
    terms: Vec<Term>,
}
impl Polynomial {
    pub fn from_values(values: &Vec<Ratio<isize>>) -> Option<Polynomial> {
        if values.len() < 2 {
            None
        }
        else if util::all_equal(values) {
            Some(Polynomial {
                terms: vec![Term {
                    coefficient: values[0],
                    exponent: 0
                }]
            })
        }
        else {
            let mut p = Polynomial {
                terms: vec![Term::from_values(values)?],
            };
            if p.terms[0].coefficient != num::zero() {
                let mut divided = Vec::new();
                for i in values.iter().enumerate() {
                    divided.push(i.1 - p.terms[0].apply(&(i.0 as isize)));
                }
                p.terms.append(&mut Polynomial::from_values(&divided).unwrap_or_else(|| Polynomial { terms: Vec::new() }).terms);
            }
            Some(p)
        }
    }
}
impl FmtAble for Polynomial {
    fn format(&self, f: &impl FmtEr) -> String {
        let mut iter = self.terms.iter();
        let mut s: String = iter.next().unwrap().format(f);
        for term in iter {
            s = f.add(s.as_str(), term.format(f).as_str());
        }
        s
    }
}

#[derive(Debug)]
struct Term {
    pub coefficient: Ratio<isize>,
    pub exponent: u8,
}
impl Term {
    pub fn from_values(values: &Vec<Ratio<isize>>) -> Option<Term> {
        if values.len() < 3 {
            return None
        }
        let mut diffs: Vec<Ratio<isize>> = Vec::new();
        for i in 0..values.len() - 1 {
            diffs.push(values[i + 1] - values[i]);
        };
        if util::all_equal(&diffs) {
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
/*impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format(&Unicode))
    }
}*/
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
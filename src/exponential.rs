use num::rational::Ratio;
use crate::fmt::{FmtEr, FmtAble};

pub struct Exponential {
    stretch: Ratio<i32>,
    ratio: Ratio<i32>,
    asymptote: Ratio<i32>,
}
impl Exponential {
    pub fn from_values(values: &Vec<Ratio<i32>>) -> Option<Self> {
        if values.len() < 3 {
            None
        }
        else {
            let ratio = (values[2] - values[1]) / (values[1] - values[0]);
            let stretch = (values[1] - values[0]) / (ratio - 1);
            let asymptote = values[0] - stretch;
            let new = Self {
                stretch,
                ratio,
                asymptote,
            };
            for value in values.iter().enumerate() {
                if value.1 != &new.apply(&(value.0 as i32)) {
                    return None;
                }
            }
            Some(new)
        }
    }
    fn apply(&self, value: &i32) -> Ratio<i32> {
        self.stretch * self.ratio.pow(*value) + self.asymptote
    }
}
impl FmtAble for Exponential {
    fn format(&self, f: &impl FmtEr) -> String {
        let mut s ;
        if f.multiply("a", "(b)") == "a(b)" { 
            s = format!("({})", self.ratio);
        }   
        else { 
            s = self.ratio.to_string();
        }
        s = f.pow(s.as_str(), f.x().to_string().as_str());
        if self.stretch != num::one() {
            s = f.multiply(self.stretch.to_string().as_str(), s.as_str());
        }
        if self.asymptote != num::zero() {
            s = f.add(s.as_str(), self.asymptote.to_string().as_str())
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn ratios() {
        assert!(all_equal(&Exponential::from_values(&as_ratios(vec![1, 2, 4, 8, 16]))));
    }

    // TODO tests
}
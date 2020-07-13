use num::rational::Ratio;
use crate::fmt::{FmtEr, FmtAble};
use crate::util::all_equal;

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
            let mut ratios = Vec::new();
            for i in 0..values.len() - 1 {
                ratios.push(values[i + 1] / values[i]);
            }
            if all_equal(&ratios) {
                let asymptote = values[0] - values[1];
                Some(Self {
                    stretch: values[1] - values[0], // Simplify
                    ratio: ratios[0],
                    asymptote: values[0] - values[1],
                })
            }
            else {
                None
            }
        }
    }
}
impl FmtAble for Exponential {
    fn format(&self, f: &impl FmtEr) -> String {
        // do stuff
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
}
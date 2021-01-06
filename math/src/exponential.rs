use num::rational::Ratio;
use config::fmt::{FmtEr, FmtAble};

pub struct Exponential {
    stretch: Ratio<i32>,
    ratio: Ratio<i32>,
    asymptote: Ratio<i32>,
}
impl Exponential {
    pub fn from_values(values: &Vec<Ratio<i32>>) -> Option<Self> {
        if values.len() < 3 || values[0] == values[1] {
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
        let mut s;
        if self.stretch != num::one() && f.multiply("a", "(b)") == "a(b)" || self.ratio.denom() != &1 { 
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
    use super::Exponential;
    use crate::util::as_ratios;
    use config::fmt::{formatters, FmtAble};
    use num::rational::Ratio;
    const ASCII: formatters::ASCII = formatters::ASCII;

    #[test]
    fn parent_base2() {
        assert_eq!(Exponential::from_values(&as_ratios(vec![1, 2, 4])).unwrap().format(&ASCII), "2^x");
    }

    #[test]
    fn parent_base3() {
        assert_eq!(Exponential::from_values(&as_ratios(vec![1, 3, 9])).unwrap().format(&ASCII), "3^x");
    }

    #[test]
    fn decay() {
        assert_eq!(Exponential::from_values(&vec![num::one(), Ratio::new(1, 2), Ratio::new(1, 4)]).unwrap().format(&ASCII), "(1/2)^x");
    }

    #[test]
    fn translate() {
        assert_eq!(Exponential::from_values(&as_ratios(vec![2, 3, 5])).unwrap().format(&ASCII), "2^x+1");
    }

    #[test]
    fn stretch() {
        assert_eq!(Exponential::from_values(&as_ratios(vec![2, 4, 8])).unwrap().format(&ASCII), "2*2^x");
    }

    #[test]
    fn all() {
        assert_eq!(Exponential::from_values(&as_ratios(vec![5, 9, 17])).unwrap().format(&ASCII), "4*2^x+1");
    }

    #[test]
    fn unicode() {
        assert_eq!(Exponential::from_values(&as_ratios(vec![2, 4, 8])).unwrap().format(&formatters::Unicode), "2(2)ˣ");
    }
}
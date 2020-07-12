use std::{io::{self, Write}, convert::TryInto, num::ParseIntError};
use num::rational::Ratio;
use regex::Regex;

pub fn get_pattern() -> Vec<Ratio<i32>> {
    let mut pattern = String::new();
    print!("Pattern: ");
    io::stdout().flush().expect("Unable to flush buffer");
    io::stdin()
        .read_line(&mut pattern)
        .expect("Could not read user input");
    let parsed = pattern.split(',').map(parse);
    let mut vec: Vec<Ratio<i32>> = Vec::new();
    for p in parsed {
        match p {
            Ok(ratio) => vec.push(ratio),
            Err(err) => {
                println!("Error: {}", err);
                return get_pattern();
            }
        }
    }
    vec
}

fn parse(mut s: &str) -> Result<Ratio<i32>, ParseIntError> {
    s = s.trim();
    if Regex::new(r"^-?\d*\.?\d+$").unwrap().is_match(s) {
        Ok(match s.find('.') {
            // The reason I don't just use Ratio::from_float(s.parse().unwrap()) is because of roundoff errors.
            // For example, Ratio::from_float(0.3).unwrap() = 5,404,319,552,844,595/18,014,398,509,481,984
            Some(mut index) => {
                let negative = s.starts_with('-');
                if negative {
                    s = &s[1..s.len()];
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
        })
    }
    else if Regex::new(format!(r"^-?(?:\d+\s+)?\d\s*/\s*-?\d$").as_str()).unwrap().is_match(s) {
        let number_regex =  Regex::new(r"-?\d").unwrap();
        let mut matches: Vec<_> = number_regex.find_iter(s).collect();
        let whole;
        if matches.len() == 3 {
            whole = Ratio::from_integer(matches.remove(0).as_str().parse()?);
        }
        else {
            whole = num::zero();
        }
        assert_eq!(matches.len(), 2);
        Ok(whole + Ratio::new(if s.starts_with('-') { -1 } else { 1 } * num::abs(matches[0].as_str().parse::<i32>()?), matches[1].as_str().parse()?))
    }
    else {
        match s.parse::<u32>() {
            Ok(_parsed) => panic!("Valid number not matched"),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_number() {
        assert_eq!(parse("2").unwrap(), Ratio::from_integer(2));
    }

    #[test]
    fn decimal() {
        assert_eq!(parse("1.3").unwrap(), Ratio::new(13, 10));
    }

    #[test]
    fn fraction() {
        assert_eq!(parse("1 / 2").unwrap(), Ratio::new(1, 2));
    }

    #[test]
    fn mixed_number() {
        assert_eq!(parse("1 1/2").unwrap(), Ratio::new(3, 2));
    }

    #[test]
    fn negative() {
        assert_eq!(parse("-2").unwrap(), Ratio::from_integer(-2));
    }

    #[test]
    fn negative_decimal() {
        assert_eq!(parse("-1.3").unwrap(), Ratio::new(-13, 10));
    }

    #[test]
    fn negative_fraction() {
        assert_eq!(parse("-1 / 2").unwrap(), Ratio::new(-1, 2));
    }

    #[test]
    fn negative_mixed_number() {
        assert_eq!(parse("-1 1/2").unwrap(), Ratio::new(-3, 2));
    }
}
use std::io::{self, Write};
use num::rational::Ratio;
use regex::Regex;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;

pub fn get_pattern() -> Vec<Ratio<isize>> {
    let mut pattern = String::new();
    print!("Pattern: ");
    io::stdout().flush().expect("Unable to flush buffer");
    io::stdin()
        .read_line(&mut pattern)
        .expect("Could not read user input");
    let parsed = pattern.split(',').map(parse);
    let mut vec: Vec<Ratio<isize>> = Vec::new();
    for p in parsed {
        match p {
            Ok(ratio) => vec.push(ratio),
            Err(err) => {
                println!("{}", err);
                return get_pattern();
            }
        }
    }
    vec
}

fn parse(mut s: &str) -> Result<Ratio<isize>, ParseNumberError> {
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
                let ratio = (   if index == 0 { num::zero() }
                    else { Ratio::from_integer((&s[0..index]).parse().unwrap())} )
                + Ratio::new(
                    (&s[index + 1..s.len()]).parse().unwrap(), 
                    10isize.pow((s.len() - index - 1).try_into().unwrap())
                );
                let neg = if negative { -1 } else { 1 };
                ratio * neg
            },
            None => Ratio::from_integer(s.parse().unwrap()),
        })
    }
    else if Regex::new(format!(r"^(?:-?\d+\s+)?-?\d\s*/\s*-?\d$",).as_str()).unwrap().is_match(s) {
        let number_regex =  Regex::new(r"-?\d").unwrap();
        let mut matches: Vec<_> = number_regex.find_iter(s).collect(); //Vec<<Matches as Iterator>::Item> = 
        let whole;

        if matches.len() == 3 {
            whole = Ratio::from_integer(matches.remove(0).as_str().parse().unwrap());
        }
        else {
            whole = num::zero();
        }
        assert_eq!(matches.len(), 2);
        Ok(whole + Ratio::new(matches[0].as_str().parse().unwrap(), matches[1].as_str().parse().unwrap()))
    }
    else {
        Err(ParseNumberError {
            number: s
        })
    }
    /*if s.contains(' ') {
        for term in s.split_whitespace() {

        }
    }
    Ratio::new(1, 2) + Ratio::new(1, 2)*/
}

#[derive(Debug)]
struct ParseNumberError<'a> {
    number: &'a str
}

impl fmt::Display for ParseNumberError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {} is not a valid number", self.number)
    }
}

impl Error for ParseNumberError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
use std::char;
use std::cmp::max;

#[allow(non_upper_case_globals)]
pub const x: char = '𝑥';

pub fn superscript(num: i32) -> String {
    let mut s = String::new();
    if num < 0 {
        s.push('⁻');
    }
    let num = num.abs() as u32;
    for i in (1..max((num as f64).log(10.0).ceil() as u32 + 1, 2)).rev() {
        let n = num % 10u32.pow(i) / 10u32.pow(i - 1);
        let unicode;
        if n == 1 {
            unicode = 0x00B9;
        }
        else if n == 2 || n == 3 {
            unicode = 0x00B0 + n;
        }
        else {
            unicode = 0x2070 + n;
        }
        s.push(char::from_u32(unicode).unwrap());
    }
    s
}
/*pub fn subscript(num: i32) -> String {
    let mut s = String::new();
    if num < 0 {
        s.push('₋');
    }
    let num = num.abs() as u32;
    for i in (1..max((num as f64).log(10.0).ceil() as u32 + 1, 2)).rev() {
        s.push(char::from_u32(0x2080 + num % 10u32.pow(i) / 10u32.pow(i - 1)).unwrap());
    }
    s
}
pub fn italicize(s: &mut String) { // This replaces letters with the unicode Mathematical Italic version of the letter
    let mut italic = String::with_capacity(s.len());

    for c in s.chars() {
        let ascii = c as u32;
        if ascii >= b'A' as u32 && ascii <= b'Z' as u32 {
            italic.push(char::from_u32(0x1D434 + ascii - (b'A' as u32)).unwrap());
        }
        else if ascii >= b'a' as u32 && ascii <= b'z' as u32 {
            italic.push(char::from_u32(0x1D44E + ascii - (b'a' as u32)).unwrap())
        }
        else {
            italic.push(c);
        }
    }

    *s = italic;
}*/


pub trait FmtAble {
    fn format(&self, f: &impl FmtEr) -> String;
}

pub trait FmtEr {
    fn add(&self, a: &str, b: &str) -> String {
        if b.starts_with('-') {
            self.subtract(a, &b[1..b.len()])
        }
        else {
            format!("{}+{}", a, b)
        }
    }
    fn subtract(&self, a: &str, b: &str) -> String {
        if b.starts_with('-') {
            self.subtract(a, &b[1..b.len()])
        }
        else {
            format!("{}-{}", a, b)
        }
    }
    fn multiply(&self, a: &str, b: &str) -> String {
        if a.ends_with('x') || b.starts_with('x') || a.ends_with(')') && b.starts_with('(') {
            format!("{}{}", a, b)
        }
        else {
            format!("{}*{}", a, b)
        }
    }
    fn divide(&self, a: &str, b: &str) -> String {
        if a == "0" {
            a.to_owned()
        }
        else {
            match b {
                "1" => a.to_owned(),
                "-1" => self.neg(a),
                _ => format!("{}/{}", a, b)
            }
        }
    }
    fn pow(&self, a: &str, b: &str) -> String {
        format!("{}^{}", a, b)
    }
    fn neg(&self, a: &str) -> String {
        if a.starts_with('-') {
            a[1..a.len()].to_string()
        }
        else {
            String::from("-") + a
        }
    }
    fn x(&self) -> char {
        'x'
    }
}

pub mod formatters {
    use super::*;

    #[derive(Debug)]
    pub struct ASCII;
    impl FmtEr for ASCII {}

    #[derive(Debug)]
    pub struct Unicode;
    impl FmtEr for Unicode {
        fn multiply(&self, a: &str, b: &str) -> String {
            if a.ends_with('x') || b.starts_with('x') || a.ends_with(')') && b.starts_with('(') {
                format!("{}{}", a, b)
            }
            else {
                format!("{}·{}", a, b)
            }
        }
        fn pow(&self, a: &str, b: &str) -> String {
            format!("{}{}", a, superscript(b.parse().unwrap()))
        }
        fn x(&self) -> char {
            x
        }
    }

    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub struct Java_JS;
    impl FmtEr for Java_JS {   
        fn multiply(&self, a: &str, b: &str) -> String {
            format!("{}*{}", a, b)
        }
        fn pow(&self, a: &str, b: &str) -> String {
            format!("Math.pow({}, {})", a, b)
        }
    }

    #[derive(Debug)]
    pub struct LaTeX;
    impl FmtEr for LaTeX {} // Wow, this is all defaults
}

#[derive(Debug)]
pub enum FmtEnum {
    _Unicode(formatters::Unicode),
    _ASCII(formatters::ASCII),
    #[allow(non_camel_case_types)]
    _Java_JS(formatters::Java_JS),
    _LaTeX(formatters::LaTeX),
}

#[allow(non_upper_case_globals)]
impl FmtEnum {
    pub const Unicode: FmtEnum = FmtEnum::_Unicode(formatters::Unicode);
    pub const ASCII: FmtEnum = FmtEnum::_ASCII(formatters::ASCII);
    pub const Java_JS: FmtEnum = FmtEnum::_Java_JS(formatters::Java_JS);
    pub const LaTeX: FmtEnum = FmtEnum::_LaTeX(formatters::LaTeX);
}
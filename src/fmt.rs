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
        if a.ends_with(self.x()) || b.starts_with(self.x()) || a.ends_with(')') && b.starts_with('(') {
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
            if a.ends_with(self.x()) || b.starts_with(self.x()) || a.ends_with(')') && b.starts_with('(') {
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
    Unicode(formatters::Unicode),
    ASCII(formatters::ASCII),
    #[allow(non_camel_case_types)]
    Java_JS(formatters::Java_JS),
    LaTeX(formatters::LaTeX),
}

#[allow(non_upper_case_globals)]
impl FmtEnum {
    /*pub const Unicode: FmtEnum = FmtEnum::_Unicode(formatters::Unicode);
    pub const ASCII: FmtEnum = FmtEnum::_ASCII(formatters::ASCII);
    pub const Java_JS: FmtEnum = FmtEnum::_Java_JS(formatters::Java_JS);
    pub const LaTeX: FmtEnum = FmtEnum::_LaTeX(formatters::LaTeX);*/
    pub fn print(&self, p: &impl FmtAble) {
        println!("{}", match self {
            FmtEnum::Unicode(e) => p.format(e),
            FmtEnum::ASCII(e) => p.format(e),
            FmtEnum::Java_JS(e) => p.format(e),
            FmtEnum::LaTeX(e) => p.format(e),
        });
    }
}
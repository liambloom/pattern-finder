mod polynomial;
mod exponential;
mod user_input;
mod fmt;
mod util;

use std::io::{stdin, stdout, Write};
use crossterm::{
    terminal::{Clear, ClearType},
    cursor::MoveUp,
    execute
};
use polynomial::Polynomial;
use exponential::Exponential;
use fmt::{FmtEnum, formatters};
use indexmap::map::IndexMap;

fn main() {
    let mut fmter_map = IndexMap::new();

    fmter_map.insert(String::from("Unicode - Best looking, but may cause rendering errors"), &FmtEnum::Unicode(formatters::Unicode));
    fmter_map.insert(String::from("ASCII - Use this to avoid rendering errors"), &FmtEnum::ASCII(formatters::ASCII));
    fmter_map.insert(String::from("Java/JS - Equations are printed as java/javascript code"), &FmtEnum::Java_JS(formatters::Java_JS));
    fmter_map.insert(String::from("LaTeX - This is the standard used by many online calculators"), &FmtEnum::LaTeX(formatters::LaTeX));

    let default_fmt = menu("Please choose a display mode", &fmter_map);

    loop {
        let pattern = user_input::get_pattern();
        match Polynomial::from_values(&pattern) {
            Some(polynomial) => default_fmt.print(&polynomial),
            None => println!("No pattern found"),
        }
    }
}

fn menu<'a, T>(prompt: &str, options: &IndexMap<String, &'a T>) -> &'a T {
    println!("{}:", prompt);
    let lines = options.len() as u16 + 1;
    let mut values = Vec::new();
    let mut err = 0;
    for option in options.into_iter().enumerate() {
        values.push((option.1).1);
        println!("{}. {}", option.0 + 1, (option.1).0);
    };
    loop {
        let mut res = String::new();
        stdin()
            .read_line(&mut res)
            .expect("Could not read user input");
        res = String::from(res.trim());
        if let Ok(i) = res.parse::<usize>() {
            if i != 0 {
                if let Some(e) = values.get(i - 1) {
                    execute!(
                        stdout(),
                        MoveUp(lines + 1 + err),
                        Clear(ClearType::FromCursorDown),
                    ).unwrap();
                    return e;
                }
            }
        }
        execute!(
            stdout(),
            MoveUp(1 + err),
            Clear(ClearType::FromCursorDown),
        ).unwrap();
        println!("{} is not an available option", res);
        err = 1;
    }
}
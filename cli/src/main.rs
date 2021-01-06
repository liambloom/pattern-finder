mod menu;

use std::io::{stdin, stdout, Write};
use crossterm::{
    terminal::{Clear, ClearType},
    cursor::MoveUp,
    execute,
};
use math::{polynomial::Polynomial, exponential::Exponential};
use config::{fmt::{FmtEnum, formatters}, output::Output};
use indexmap::map::IndexMap;
use menu::Menu;
use util::parse;
use num::rational::Ratio;

fn main() {
    println!("Setup:");
    println!("Use arrow keys to move, space to select");

    let mut fmter_map = IndexMap::new();
    fmter_map.insert(String::from("Unicode"), FmtEnum::Unicode(formatters::Unicode));
    fmter_map.insert(String::from("ASCII"), FmtEnum::ASCII(formatters::ASCII));
    fmter_map.insert(String::from("Java/JS"), FmtEnum::Java_JS(formatters::Java_JS));
    fmter_map.insert(String::from("LaTeX"), FmtEnum::LaTeX(formatters::LaTeX));
    let mut fmt_menu = Menu::new(String::from("Display mode"), fmter_map);

    let mut output_map = IndexMap::new();
    output_map.insert(String::from("Console"), Output::Console);
    output_map.insert(String::from("File"), Output::File(None));
    let mut output_menu = Menu::new(String::from("Output location"), output_map);
    
    let default_fmt = fmt_menu.get_user_input();
    let mut default_output = output_menu.get_user_input();
    let default_output_owned;
    if let Output::File(None) = default_output {
        let (path, lines) = Output::file_name_ui();
        default_output_owned = Output::File(Some(path));
        default_output = &default_output_owned;
        execute!(
            stdout(),
            MoveUp(lines),
            Clear(ClearType::FromCursorDown),
        ).unwrap();
    }

    execute!(
        stdout(),
        MoveUp(4),
        Clear(ClearType::FromCursorDown),
    ).unwrap();
    
    loop {
        let pattern = get_pattern();
        match Polynomial::from_values(&pattern, 0) {
            Some(polynomial) => default_output.print(&default_fmt.format(&polynomial)),
            None => match Exponential::from_values(&pattern) {
                Some(exponential) => default_output.print(&default_fmt.format(&exponential)),
                None => println!("No pattern found"),
            },
        }
    }
}

pub fn get_pattern() -> Vec<Ratio<i32>> {
    let mut pattern = String::new();
    print!("Pattern: ");
    stdout().flush().expect("Unable to flush buffer");
    stdin()
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
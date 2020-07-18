mod polynomial;
mod exponential;
mod user_input;
mod fmt;
mod util;
mod menu;
mod output;

use std::io::{stdout, Write};
use crossterm::{
    terminal::{Clear, ClearType},
    cursor::MoveUp,
    execute,
};
use polynomial::Polynomial;
use exponential::Exponential;
use fmt::{FmtEnum, formatters};
use indexmap::map::IndexMap;
use menu::Menu;
use output::Output;

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
        let pattern = user_input::get_pattern();
        match Polynomial::from_values(&pattern) {
            Some(polynomial) => default_output.print(&default_fmt.format(&polynomial)),
            None => match Exponential::from_values(&pattern) {
                Some(exponential) => default_output.print(&default_fmt.format(&exponential)),
                None => println!("No pattern found"),
            },
        }
    }
}
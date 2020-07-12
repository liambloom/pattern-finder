mod polynomial;
mod user_input;
mod fmt;
mod util;

use std::io::{stdout, Write};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor::MoveUp,
    execute
};
use polynomial::Polynomial;
use fmt::{FmtEnum, formatters};

fn main() {
    println!("Please choose a display mode:");
    println!("1. Unicode - Best looking, but may cause rendering errors");
    println!("2. ASCII - Use this to avoid rendering errors");
    println!("3. Java/JS - Equations are printed as java/javascript code");
    println!("4. LaTeX - This is the standard used by many online calculators");

    enable_raw_mode().unwrap();

    let default_fmt;

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => std::process::exit(1),
            Event::Key(KeyEvent {
                code: KeyCode::Char('1'),
                modifiers: _these_identifiers_are_not_defined,
            }) => {
                default_fmt = FmtEnum::Unicode(formatters::Unicode);
                break;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                modifiers: _they_should_throw_errors,
            }) => {
                default_fmt = FmtEnum::ASCII(formatters::ASCII);
                break;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('3'),
                modifiers: _why_do_these_work,
            }) => {
                default_fmt = FmtEnum::Java_JS(formatters::Java_JS);
                break;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('4'),
                modifiers: _seriously_how_does_this_compile,
            }) => {
                default_fmt = FmtEnum::LaTeX(formatters::LaTeX);
                break;
            },
            _ => (),
        }
    }

    execute!(
        stdout(),
        MoveUp(5),
        Clear(ClearType::FromCursorDown),
    ).unwrap();

    disable_raw_mode().unwrap();

    loop {
        let pattern = user_input::get_pattern();
        match Polynomial::from_values(&pattern) {
            Some(polynomial) => default_fmt.print(&polynomial),
            None => println!("No pattern found"),
        }
    }
}
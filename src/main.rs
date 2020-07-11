mod polynomial;
mod user_input;
mod util;
mod fmt;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use polynomial::Polynomial;
use fmt::{FmtAble, FmtEnum};

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
                default_fmt = FmtEnum::Unicode;
                break;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                modifiers: _they_should_throw_errors,
            }) => {
                default_fmt = FmtEnum::ASCII;
                break;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('3'),
                modifiers: _why_do_these_work,
            }) => {
                default_fmt = FmtEnum::Java_JS;
                break;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('4'),
                modifiers: _seriously_how_does_this_compile,
            }) => {
                default_fmt = FmtEnum::LaTeX;
                break;
            },
            _ => (),
        }
    }

    disable_raw_mode().unwrap();

    loop {
        let polynomial = Polynomial::from_values(&user_input::get_pattern()).unwrap();
        println!("{}", match &default_fmt {
            FmtEnum::_Unicode(r) => polynomial.format(r),
            FmtEnum::_ASCII(r) => polynomial.format(r),
            FmtEnum::_Java_JS(r) => polynomial.format(r),
            FmtEnum::_LaTeX(r) => polynomial.format(r),
        });
    }
}
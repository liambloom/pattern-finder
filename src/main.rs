mod polynomial;
mod exponential;
mod user_input;
mod fmt;
mod util;
mod menu;

use std::io::{stdout, Write};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, Clear, ClearType},
    cursor::{MoveUp, Hide, Show},
    execute,
};
use polynomial::Polynomial;
use exponential::Exponential;
use fmt::{FmtEnum, formatters};
use indexmap::map::IndexMap;
use menu::Menu;

fn main() {
    terminal::enable_raw_mode().unwrap();
    execute!(
        stdout(),
        Hide,
    ).unwrap();

    println!("Setup:");
    println!("Use arrow keys to move, space to select");

    let mut fmter_map = IndexMap::new();

    fmter_map.insert(String::from("Unicode"), &FmtEnum::Unicode(formatters::Unicode));
    fmter_map.insert(String::from("ASCII"), &FmtEnum::ASCII(formatters::ASCII));
    fmter_map.insert(String::from("Java/JS"), &FmtEnum::Java_JS(formatters::Java_JS));
    fmter_map.insert(String::from("LaTeX"), &FmtEnum::LaTeX(formatters::LaTeX));

    let mut fmt_menu = Menu::new(String::from("Display mode"), fmter_map);

    fmt_menu.show();

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE
            }) => fmt_menu.increment(),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE
            }) => fmt_menu.decrement(),
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::NONE
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                execute!(stdout(), Show);
                std::process::exit(1)
            },
            _ => (),
        }
    }

    fmt_menu.hide();
    let default_fmt = fmt_menu.value();

    terminal::disable_raw_mode().unwrap();
    execute!(
        stdout(),
        MoveUp(3),
        Clear(ClearType::FromCursorDown),
        Show,
    ).unwrap();
    
    loop {
        let pattern = user_input::get_pattern();
        match Polynomial::from_values(&pattern) {
            Some(polynomial) => default_fmt.print(&polynomial),
            None => match Exponential::from_values(&pattern) {
                Some(exponential) => default_fmt.print(&exponential),
                None => println!("No pattern found"),
            },
        }
    }
}
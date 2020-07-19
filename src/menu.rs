use std::io::{stdout, Write};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, Clear, ClearType},
    style::Styler,
    cursor::{self, MoveTo, Hide, Show, SavePosition, RestorePosition},
    execute,
};
use indexmap::map::IndexMap;

pub struct Menu<T> {
    pub items: IndexMap<String, T>,
    pub i: usize,
    pub line: u16,
    pub visible: bool,
    pub name: String,
    pub focus: bool,
}
impl<T> Menu<T> {
    pub fn new(name: String, map: IndexMap<String, T>) -> Self {
        Self {
            items: map,
            i: 0,
            line: cursor::position().unwrap().1,
            visible: false,
            name,
            focus: false,
        }
    }
    pub fn increment(&mut self) {
        if self.i >= self.items.len() - 1 {
            self.i = 0;
        }
        else {
            self.i += 1;
        }
        self.print();
    }
    pub fn decrement(&mut self) {
        if self.i == 0 {
            self.i = self.items.len() - 1;
        }
        else { 
            self.i -= 1;
        }
        self.print();
    }
    pub fn show(&mut self) {
        self.line = cursor::position().unwrap().1;
        self.visible = true;
        self.focus = true;
        self.print();
    }
    pub fn value(&self) -> &T {
        self.items.get_index(self.i).unwrap().1
    }
    fn print(&self) {
        execute!(
            stdout(),
            SavePosition,
            MoveTo(0, self.line),
            Clear(ClearType::CurrentLine)
        ).unwrap();
        print!("{}: ", self.name);
        for item in self.items.iter().enumerate() {
            print!("{}  ", 
                if item.0 == self.i { item.1 .0.as_str().reverse() }
                else { item.1 .0.as_str().reset() }
            )
        }
        stdout().flush().unwrap();
        execute!(stdout(), RestorePosition).unwrap();
    }
    pub fn get_user_input(&mut self) -> &T {
        terminal::enable_raw_mode().unwrap();
        execute!(
            stdout(),
            Hide,
        ).unwrap();

        //println!("Use arrow keys to move, space to select");

        self.show();

        loop {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                }) => self.increment(),

                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                }) => self.decrement(),

                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                }) => break,

                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => {
                    execute!(stdout(), Show).unwrap();
                    std::process::exit(1);
                },

                _ => (),
            }
        }

        //self.hide();
        
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), Show).unwrap();
        println!();
        
        self.value()
    }
}

/*pub fn menu() {
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

    let fmter_menu = Menu::new(String::from("Display mode"), fmter_map);

    terminal::disable_raw_mode().unwrap();
    execute!(
        stdout(),
        MoveUp(2),
        Clear(ClearType::FromCursorDown),
        Show,
    ).unwrap();
}*/
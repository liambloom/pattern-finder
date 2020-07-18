use std::io::{stdout, Write};
use crossterm::{
    terminal::{Clear, ClearType},
    style::Styler,
    cursor::{self, MoveTo, SavePosition, RestorePosition},
    execute,
};
use indexmap::map::IndexMap;

pub struct Menu<T> { // TODO allow for multiple menus simultaneously (maybe represent cursor with underline?)
    pub items: IndexMap<String, T>,
    pub i: usize,
    pub line: u16,
    pub visible: bool,
    pub name: String,
}
impl<T> Menu<T> {
    pub fn new(name: String, map: IndexMap<String, T>) -> Self {
        Self {
            items: map,
            i: 0,
            line: cursor::position().unwrap().1,
            visible: false,
            name
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
        self.visible = true;
        self.print();
    }
    pub fn hide(&mut self) {
        self.visible = false;
        execute!(
            stdout(),
            SavePosition,
            MoveTo(0, self.line),
            Clear(ClearType::CurrentLine),
            RestorePosition,
        ).unwrap();
    }
    pub fn value(&self) -> &T {
        self.items.get_index(self.i).unwrap().1
    }
    fn print(&self) {
        execute!(
            stdout(),
            SavePosition,
            MoveTo(0, self.line),
            Clear(ClearType::CurrentLine),
        ).unwrap();
        print!("{}: ", self.name);
        for item in self.items.iter().enumerate() {
            print!("{}  ", 
                if item.0 == self.i { item.1 .0.as_str().reverse() }
                else { item.1 .0.as_str().reset() }
            )
        }
        stdout().flush().unwrap();
        execute!(
            stdout(),
            RestorePosition,
        ).unwrap();
    }
}
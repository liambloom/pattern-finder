use std::io::{self, Write};
use std::fs;
use std::path::Path;

pub enum Output {
    Console,
    File(Option<String>),
}
impl Output {
    pub fn print(&self, s: &str) {
        match self {
            Self::Console => println!("{}", s),
            Self::File(path) => {
                writeln!(
                    fs::OpenOptions::new()
                        .append(true)
                        .open(path.as_ref().unwrap()).unwrap(),
                    "{}", s).unwrap();
            }
        }
    }
    pub fn file_name_ui() -> (String, u16) {
        let mut path = String::new();
        let mut lines = 0;
        while !Path::new(&path).exists() {
            if !path.is_empty() {
                println!(r#"File "{}" not found"#, path);
                lines += 1;
            }
            path.clear();
            print!("Please enter file path: ");
            lines += 1;
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut path).expect("Could not read input");
            path = String::from(path.trim()); //foo2
        }
        (path, lines)
    }
}
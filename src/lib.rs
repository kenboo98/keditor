extern crate termion;

use std::fs::File;
use std::io::{Read, stdin, Stdin, stdout, Stdout, Write};
use std::io;

// Import the color module.
use termion::{clear, cursor};
use termion::event::Key;
use termion::input::{TermRead, Keys};
use termion::raw::{IntoRawMode, RawTerminal};

enum CursorDir {
    Up,
    Down,
    Left,
    Right,
}

struct IntermediateFile {
    lines: Vec<String>
}

struct Editor {
    intermediate_file: IntermediateFile,
    cursor_col: u16,
    cursor_row: u16,
}

impl Editor {
    fn new(file: IntermediateFile) -> Editor {
        Editor {
            intermediate_file: file,
            cursor_col: 1,
            cursor_row: 1,
        }
    }

    fn move_cursor(&mut self, dir: Key) {
        match dir {
            Key::Left => {
                if self.cursor_col > 1 {
                    self.cursor_col -= 1
                }
            }
            Key::Right => {
                if self.intermediate_file.lines[self.cursor_row as usize - 1].len() >= self.cursor_col as usize {
                    self.cursor_col += 1
                } else if self.cursor_row < self.intermediate_file.lines.len() as u16 {
                    self.cursor_col = 0;
                    self.cursor_row += 1;
                }
            }
            Key::Down => {
                if self.cursor_row < self.intermediate_file.lines.len() as u16 {
                    self.cursor_row += 1
                }
            }
            Key::Up => {
                if self.cursor_row > 1 {
                    self.cursor_row -= 1
                }
            }
            _ => {}
        }
    }

    fn print_lines(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        for (i, line) in self.intermediate_file.lines.iter().enumerate() {
            write!(stdout, "{}{}", line, termion::cursor::Goto(1, (i + 2) as u16));
        }
        write!(stdout, "{}", termion::cursor::Goto(self.cursor_col, self.cursor_row)).unwrap();
    }

    fn clear(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", termion::clear::All);
    }
}


pub fn run(mut args: std::env::Args) -> Result<(), &'static str> {
    args.next();
    let filename = match args.next() {
        Some(name) => name,
        None => return Err("Not enough arguments")
    };

    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => { return Err("Could not open file"); }
    };

    let mut buffer = Vec::new();
    loop {
        match file.read_line() {
            Ok(Some(line)) => {
                if line.is_empty() {
                    break;
                }
                buffer.push(line);
            }
            _ => { break; }
        }
    }
    let intermediate_file = IntermediateFile { lines: buffer };

    let mut editor = Editor::new(intermediate_file);


    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1)).unwrap();


    for (i, line) in editor.intermediate_file.lines.iter().enumerate() {
        write!(stdout, "{}{}", line, termion::cursor::Goto(1, (i + 2) as u16));
    }
    write!(stdout, "{}", termion::cursor::Goto(editor.cursor_col, editor.cursor_row)).unwrap();

    stdout.flush().unwrap();


    let stdin = stdin();

    for c in stdin.keys() {
        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char(c) => print!("{}", c),
            Key::Alt(c) => print!("Alt-{}", c),
            Key::Ctrl('c') => {
                editor.clear(&mut stdout);
                return Ok(());
            }
            Key::Left => editor.move_cursor(Key::Left),
            Key::Right => editor.move_cursor(Key::Right),
            Key::Up => editor.move_cursor(Key::Up),
            Key::Down => editor.move_cursor(Key::Down),
            _ => print!("Other"),
        }


        write!(stdout, "{}",
               // Clear the screen.
               termion::clear::All).unwrap();

        editor.print_lines(&mut stdout);
        stdout.flush().unwrap();
    }
    Ok(())
}
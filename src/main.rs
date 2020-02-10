extern crate termion;

use std::env::args;
use std::fs::File;
use std::io::{Read, stdin, stdout, Write, Stdin, Stdout};

// Import the color module.
use termion::{clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

enum CursorDir {
    up,
    down,
    left,
    right,
}

fn move_cursor(std_out: &mut RawTerminal<Stdout>, dir: CursorDir) {
    match dir {
        CursorDir::left => {
            write!(std_out, "{}",
                   termion::cursor::Left(1)).unwrap();
        }
        CursorDir::right => {
            write!(std_out, "{}",
                   termion::cursor::Right(1)).unwrap();
        }
        CursorDir::down => {
            write!(std_out, "{}",
                termion::cursor::Down(1)).unwrap();
        }
        CursorDir::up => {
            write!(std_out, "{}",
                   termion::cursor::Up(1)).unwrap();
        }
    }
}

fn run(mut args: std::env::Args) -> Result<(), &'static str> {
    args.next();
    let filename = match args.next() {
        Some(name) => name,
        None => return Err("Not enough arguments")
    };

    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => { return Err("Could not open file"); }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1)).unwrap();
    println!("{}", contents);


    loop {
        let stdin = stdin();
        for c in stdin.keys() {
            // Print the key we type...
            match c.unwrap() {
                // Exit.
                Key::Char(c) => print!("{}", c),
                Key::Alt(c) => print!("Alt-{}", c),
                Key::Ctrl('c') => { return Ok(()); }
                Key::Left => move_cursor(&mut stdout, CursorDir::left),
                Key::Right => move_cursor(&mut stdout, CursorDir::right),
                Key::Up => move_cursor(&mut stdout, CursorDir::up),
                Key::Down => move_cursor(&mut stdout, CursorDir::down),
                Key::Backspace => print!("\r"),
                _ => print!("Other"),
            }

            // Flush again.
            stdout.flush().unwrap();
        }
    }
}

fn main() {
    run(args());
}
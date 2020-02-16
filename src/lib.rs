extern crate termion;

use std::fs::File;
use std::io::{Read, stdin, Stdin, stdout, Stdout, Write};
use std::io;

// Import the color module.
use termion::{clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

enum CursorDir {
    Up,
    Down,
    Left,
    Right,
}

fn move_cursor(std_out: &mut RawTerminal<Stdout>, dir: CursorDir) {
    match dir {
        CursorDir::Left => {
            write!(std_out, "{}",
                   termion::cursor::Left(1)).unwrap();
        }
        CursorDir::Right => {
            write!(std_out, "{}",
                   termion::cursor::Right(1)).unwrap();
        }
        CursorDir::Down => {
            write!(std_out, "{}",
                   termion::cursor::Down(1)).unwrap();
        }
        CursorDir::Up => {
            write!(std_out, "{}",
                   termion::cursor::Up(1)).unwrap();
        }
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
            _ => {break;}
        }
    }


    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1)).unwrap();

    for (i, line) in buffer.iter().enumerate() {
        write!(stdout, "{}{}", line, termion::cursor::Goto(1, (i+2) as u16));

    }


    loop {
        let stdin = stdin();
        for c in stdin.keys() {
            // Print the key we type...
            match c.unwrap() {
                // Exit.
                Key::Char(c) => print!("{}", c),
                Key::Alt(c) => print!("Alt-{}", c),
                Key::Ctrl('c') => { return Ok(()); }
                Key::Left => move_cursor(&mut stdout, CursorDir::Left),
                Key::Right => move_cursor(&mut stdout, CursorDir::Right),
                Key::Up => move_cursor(&mut stdout, CursorDir::Up),
                Key::Down => move_cursor(&mut stdout, CursorDir::Down),
                Key::Backspace => print!("\r"),
                _ => print!("Other"),
            }

            // Flush again.
            stdout.flush().unwrap();
        }
    }
}
use termion::raw::RawTerminal;
use std::io::{Stdout, Write};
use termion::event::Key;

use crate::intermediate::IntermediateFile;
use std::cmp::max;

pub struct Editor {
    pub intermediate_file: IntermediateFile,
    pub cursor_col: u16,
    pub cursor_row: u16,
    pub line_number: u64
}

impl Editor {
    pub fn new(file: IntermediateFile) -> Editor {
        Editor {
            intermediate_file: file,
            cursor_col: 1,
            cursor_row: 1,
            line_number: 0
        }
    }

    pub fn move_cursor(&mut self, dir: Key) {
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
                    self.line_number += 1;
                }
            }
            Key::Down => {
                if self.cursor_row < self.intermediate_file.lines.len() as u16 {
                    self.cursor_row += 1;
                    self.line_number += 1;
                    if self.cursor_col > self.intermediate_file.lines[self.line_number as usize].len() as u16 {
                        self.cursor_col = max((self.intermediate_file.lines[self.line_number as usize].len() + 1) as u16, 1);
                    }

                }
            }
            Key::Up => {
                if self.cursor_row > 1 {
                    self.cursor_row -= 1;
                    self.line_number -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn print_lines(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        for (i, line) in self.intermediate_file.lines.iter().enumerate() {
            write!(stdout, "{}{}", line, termion::cursor::Goto(1, (i + 2) as u16)).unwrap();
        }
        write!(stdout, "{}", termion::cursor::Goto(self.cursor_col, self.cursor_row)).unwrap();
    }

    pub fn clear(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn new_line(&mut self) {
        let remaining = match self.intermediate_file.lines[self.line_number as usize].len() {
            0 => String::new(),
            _ => self.intermediate_file.lines[self.line_number as usize].split_off((self.cursor_col - 1) as usize)
        };

        self.intermediate_file.lines.insert((self.line_number + 1) as usize, remaining);
        self.cursor_row += 1;
        self.cursor_col = 1;
        self.line_number += 1;
        if self.line_number == self.intermediate_file.lines.len() as u64 {
           self.intermediate_file.lines.push(String::new());
        }
    }
    pub fn write_char(&mut self, c: char) {
        if self.intermediate_file.lines[self.line_number as usize].len() == 0 {
            self.intermediate_file.lines[self.line_number as usize].push(c);
        } else {
            self.intermediate_file.lines[self.line_number as usize].insert((self.cursor_col - 1) as usize, c);
        }
        self.cursor_col += 1;
    }
}
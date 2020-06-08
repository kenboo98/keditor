use termion::raw::RawTerminal;
use std::io::{Stdout, Write};
use termion::event::Key;

use crate::intermediate::IntermediateFile;
use std::cmp::max;

pub struct Editor {
    pub file: IntermediateFile,
    pub cursor_col: u16,
    pub cursor_row: u16,
    pub file_row: u64,
    pub file_col: u64
}

impl Editor {
    pub fn new(file: IntermediateFile) -> Editor {
        Editor {
            file,
            cursor_col: 1,
            cursor_row: 1,
            file_row: 0,
            file_col: 0,

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
                if self.file.lines[self.cursor_row as usize - 1].len() >= self.cursor_col as usize {
                    self.cursor_col += 1
                } else if self.cursor_row < self.file.lines.len() as u16 {
                    self.cursor_col = 0;
                    self.cursor_row += 1;
                    self.file_row += 1;
                }
            }
            Key::Down => {
                if self.cursor_row < self.file.lines.len() as u16 {
                    self.cursor_row += 1;
                    self.file_row += 1;
                    if self.cursor_col > self.file.lines[self.file_row as usize].len() as u16 {
                        self.cursor_col = max((self.file.lines[self.file_row as usize].len() + 1) as u16, 1);
                    }

                }
            }
            Key::Up => {
                if self.cursor_row > 1 {
                    self.cursor_row -= 1;
                    self.file_row -= 1;
                    if self.cursor_col > self.file.lines[self.file_row as usize].len() as u16 {
                        self.cursor_col = max((self.file.lines[self.file_row as usize].len() + 1) as u16, 1);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn print_lines(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        for (i, line) in self.file.lines.iter().enumerate() {
            write!(stdout, "{}{}", line, termion::cursor::Goto(1, (i + 2) as u16)).unwrap();
        }
        write!(stdout, "{}", termion::cursor::Goto(self.cursor_col, self.cursor_row)).unwrap();
    }

    pub fn clear(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn new_line(&mut self) {
        let remaining = match self.file.lines[self.file_row as usize].len() {
            0 => String::new(),
            _ => self.file.lines[self.file_row as usize].split_off((self.cursor_col - 1) as usize)
        };

        self.file.lines.insert((self.file_row + 1) as usize, remaining);
        self.cursor_row += 1;
        self.cursor_col = 1;
        self.file_row += 1;
        if self.file_row == self.file.lines.len() as u64 {
           self.file.lines.push(String::new());
        }
    }
    pub fn write_char(&mut self, c: char) {
        if self.file.lines[self.file_row as usize].len() == 0 {
            self.file.lines[self.file_row as usize].push(c);
        } else {
            self.file.lines[self.file_row as usize].insert((self.cursor_col - 1) as usize, c);
        }
        self.cursor_col += 1;
    }

    pub fn back_space(&mut self) {
        if self.cursor_col > 1 {
            self.cursor_col -= 1;
            self.file.lines[self.file_row as usize].remove((self.cursor_col - 1) as usize);
        } else if self.cursor_row > 1 {
            let removed = self.file.lines.remove(self.file_row as usize);
            self.cursor_row -= 1;
            self.file_row -= 1;
            self.cursor_col = (self.file.lines[self.file_row as usize].len() as u16) + 1;
            self.file.lines[self.file_row as usize].push_str(removed.as_str());
        }
    }

    pub fn save(&mut self) {
        self.file.save();
    }
}
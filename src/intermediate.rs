use std::fs::{File, OpenOptions};
use termion::input::TermRead;
use std::io::Write;

pub struct IntermediateFile {
    pub lines: Vec<String>,
    pub file_name: String,
}

impl IntermediateFile {
    pub fn open(file_name: String) -> Result<IntermediateFile, &'static str> {
        let mut file = match File::open(&file_name) {
            Ok(f) => f,
            Err(_) => { return Err("Could not open file"); }
        };

        let mut lines = Vec::new();
        loop {
            match file.read_line() {
                Ok(Some(line)) => {
                    if line.is_empty() {
                        break;
                    }
                    lines.push(line);
                }
                _ => { break; }
            }
        }
        Ok(
            IntermediateFile {
                lines,
                file_name,
            }
        )
    }

    pub fn save(&mut self) {
        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.file_name).unwrap();
        for line in &self.lines {
            write!(file, "{}\n", line);
        }
        file.flush();

    }
}
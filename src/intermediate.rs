use std::fs::File;
use termion::input::TermRead;

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
}
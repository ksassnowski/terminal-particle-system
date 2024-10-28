use std::{
    fmt::Write,
    io::{Stdout, Write as IOWrite},
};

pub struct Screen {
    buffer: Vec<String>,
    cols: usize,
    rows: usize,
    stream: Stdout,
}

impl Screen {
    pub fn new(mut stream: Stdout, cols: usize, rows: usize) -> Screen {
        // Clear screen
        stream.write_all(b"\x1b[2J").unwrap();

        Screen {
            buffer: vec![" ".to_string(); cols * rows],
            cols,
            rows,
            stream,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = " ".to_string();
        }
    }

    pub fn write_char(&mut self, char: char, color: String, row: i32, col: i32) {
        if row >= self.rows as i32 || row < 0 || col >= self.cols as i32 || col < 0 {
            return;
        }
        self.buffer[self.cols * row as usize + col as usize] =
            format!("\x1b[38;5;{}m{}\x1b[0m", color, char);
    }

    pub fn flush(&mut self) {
        let mut output = String::new();

        output.write_str("\x1b[0;0H").unwrap();

        self.buffer.chunks(self.cols).for_each(|chars| {
            output.write_str("\r").unwrap();
            for c in chars {
                output.write_str(c).unwrap();
            }
            output.write_str("\x1b[1B").unwrap();
        });

        self.stream.write_all(output.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}

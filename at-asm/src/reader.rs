use std::io::{BufRead as _, BufReader, Read};

pub struct Reader<R> {
    reader: BufReader<R>,
    buf: String,
}

impl<R: Read> Reader<R> {
    pub fn new(reader: R) -> Self {
        Reader {
            reader: BufReader::new(reader),
            buf: String::new(),
        }
    }

    pub fn next_line(&mut self) -> Option<&str> {
        self.buf.clear();

        if self.reader.read_line(&mut self.buf).unwrap() == 0 {
            return None; // EOF
        }

        Some(self.buf.as_str())
    }
}

use std::io::{self, BufRead, StdinLock, StdoutLock};

#[derive(Debug)]
pub struct Environment {
    reader: LineReader<StdinLock<'static>>,
    stdout: StdoutLock<'static>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            reader: LineReader::new(io::stdin().lock()),
            stdout: io::stdout().lock(),
        }
    }

    pub fn reader(&mut self) -> &mut LineReader<StdinLock<'static>> {
        &mut self.reader
    }

    pub fn output(&mut self) -> &mut StdoutLock<'static> {
        &mut self.stdout
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct LineReader<R> {
    reader: R,
    buffer: String,
}

impl<R: BufRead> LineReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
        }
    }

    pub fn read_line(&mut self) -> io::Result<Option<&str>> {
        self.buffer.clear();

        if self.reader.read_line(&mut self.buffer)? == 0 {
            return Ok(None);
        }

        Ok(Some(self.buffer.trim()))
    }
}

#[derive(Debug)]
pub struct Args<'a> {
    tokens: std::str::SplitWhitespace<'a>,
}

impl<'a> Args<'a> {
    pub fn parse(input: &'a str) -> Self {
        Self {
            tokens: input.split_whitespace(),
        }
    }
}

impl<'a> Iterator for Args<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()
    }
}

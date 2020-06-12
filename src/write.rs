//! `Write` implementation that will write the output using a given format.

use crate::app::OutputFormat;
use std::io::Write;

pub struct FormatWriter {
    format: OutputFormat,
    writer: Box<dyn Write>,
}

impl FormatWriter {
    pub fn new(write: Box<dyn Write>, format: OutputFormat) -> Self {
        Self {
            format,
            writer: write,
        }
    }
}

impl Write for FormatWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.format {
            OutputFormat::Raw => self.writer.write(buf),
            OutputFormat::Hex => self.write(hex::encode(buf).as_bytes()),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

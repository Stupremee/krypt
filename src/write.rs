//! `Write` implementation that will write the output using a given format.

use crate::app::OutputFormat;
use std::io::Write;

pub struct FormatWriter<W: Write> {
    format: OutputFormat,
    writer: W,
}

impl<W: Write> FormatWriter<W> {
    pub fn new(write: W, format: OutputFormat) -> Self {
        Self {
            format,
            writer: write,
        }
    }
}

impl<W: Write> Write for FormatWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.format {
            OutputFormat::Raw => self.writer.write(buf),
            OutputFormat::Hex => {
                // FIXME: pls fix this. This is not a good way to implement
                // write but it should work for now.
                let bytes = hex::encode(buf);
                write!(self.writer, "{}", bytes)?;
                Ok(bytes.as_bytes().len())
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

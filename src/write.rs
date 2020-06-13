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
            ref format => {
                let bytes = match format {
                    OutputFormat::UpperHex => hex::encode_upper(buf),
                    OutputFormat::Hex => hex::encode(buf),
                    _ => unreachable!(),
                };
                write!(self.writer, "{}", bytes)?;
                Ok(bytes.as_bytes().len())
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

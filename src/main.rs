#![warn(rust_2018_idioms)]

mod app;
mod log;
mod write;

use app::{Mode, Options, OutputFormat};
use krypt::{chunk::ChunkRead, hash};
use std::{
    fs::File,
    io::{self, prelude::*, BufReader, BufWriter},
    path::PathBuf,
};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    match try_main() {
        Ok(()) => {}
        Err(err) => {
            error!("Error", "occurred {}", err);
            std::process::exit(1);
        }
    }
}

fn try_main() -> Result<()> {
    // Enable ansi support if target is windows.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    // Enable colored backtrace
    color_backtrace::install();

    let opt = Options::from_args();
    let mode = opt.mode;
    let output_format = opt.output_format;

    let mut input = get_input(opt.input)?;

    let output_format = output_format.unwrap_or_else(|| get_default_format(&mode));
    let mut output = get_output(opt.output, output_format)?;

    let res = match mode {
        Mode::Hash(mode) => hash::hash_with_algorithm(mode.algorithm, &mut input, &mut output),
    };
    res
}

fn get_default_format(mode: &Mode) -> OutputFormat {
    match mode {
        Mode::Hash(_) => OutputFormat::Hex,
    }
}

fn get_output(path: Option<PathBuf>, format: OutputFormat) -> Result<Box<dyn Write>> {
    if let Some(path) = path {
        let file = File::open(path)?;
        let writer = BufWriter::new(file);
        let writer = write::FormatWriter::new(writer, format);
        Ok(Box::new(writer))
    } else {
        let stdout = io::stdout();
        let writer = BufWriter::new(stdout);
        let writer = write::FormatWriter::new(writer, format);
        Ok(Box::new(writer))
    }
}

fn get_input(path: Option<PathBuf>) -> Result<ChunkRead<Box<dyn Read>>> {
    if let Some(path) = path {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(ChunkRead::new(Box::new(reader)))
    } else {
        let stdin = io::stdin();
        Ok(ChunkRead::new(Box::new(stdin)))
    }
}

#![warn(rust_2018_idioms)]

mod app;
mod log;
mod write;

use app::{HashMode, Mode, Options, OutputFormat};
use krypt::chunk::ChunkRead;
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
    let input = get_input(opt.input)?;
    let output = get_output(opt.output)?;

    let format = match opt.output_format {
        Some(f) => f,
        None => match opt.mode {
            Mode::Hash(_) => OutputFormat::Hex,
        },
    };
    let output = Box::new(write::FormatWriter::new(output, format));

    let res = match opt.mode {
        Mode::Hash(mode) => hash_data(mode, input, output),
    };
    res
}

fn hash_data(
    mode: HashMode,
    input: ChunkRead<Box<dyn Read>>,
    output: Box<dyn Write>,
) -> Result<()> {
    Ok(())
}

fn get_output(path: Option<PathBuf>) -> Result<Box<dyn Write>> {
    if let Some(path) = path {
        let file = File::open(path)?;
        Ok(Box::new(BufWriter::new(file)))
    } else {
        let stdout = io::stdout();
        Ok(Box::new(BufWriter::new(stdout)))
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

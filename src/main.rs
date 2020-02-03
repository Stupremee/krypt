use clap::arg_enum;
use krypt::{
    encode::{self, Encoding},
    hash::{self, HashAlgorithm},
};
use std::{fs::File, io::prelude::*, io::Read, path::PathBuf};
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum OutputFormat {
        Raw,
        Hex,
        HexDump,
    }
}

#[derive(StructOpt, Debug)]
enum Mode {
    /// Hashes the input value
    Hash(HashMode),
    /// Encode or decode the input data
    Encode(EncodeMode),
}

#[derive(StructOpt, Debug)]
struct Options {
    /// Input file. If no input file is provided krypt will read from stdin.
    #[structopt(name = "FILE", parse(from_os_str))]
    input: Option<PathBuf>,
    /// Specifies the output format.
    #[structopt(short = "f", long = "format", possible_values = &OutputFormat::variants(), default_value = "Raw", case_insensitive = true)]
    output_format: OutputFormat,
    /// Which operation should be executed.
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(StructOpt, Debug)]
struct EncodeMode {
    /// Sepcifies the enoding base to use.
    #[structopt(name = "BASE", possible_values = &Encoding::variants(), case_insensitive = true)]
    base: Encoding,
    /// If this option is present the input data will be decoded.
    #[structopt(short = "d", long = "decode")]
    decode: bool,
}

#[derive(StructOpt, Debug)]
struct HashMode {
    #[structopt(name = "ALGORITHM", possible_values = &HashAlgorithm::variants(), case_insensitive = true)]
    algorithm: HashAlgorithm,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(err) = try_main() {
        if atty::is(atty::Stream::Stderr) {
            eprintln!("{}", err);
        }
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let opts = Options::from_args();

    let mut file;
    let mut stdin;
    let input: &mut dyn Read = if let Some(path) = opts.input {
        file = File::open(path)?;
        &mut file
    } else {
        stdin = std::io::stdin();
        &mut stdin
    };
    let data = read_data(input)?;
    let result = match opts.mode {
        Mode::Encode(e) => {
            if e.decode {
                encode::encode_data(e.base, data)
            } else {
                encode::decode_data(e.base, data)?
            }
        }
        Mode::Hash(h) => hash::hash_with_algorithm(h.algorithm, data),
    };

    match opts.output_format {
        OutputFormat::Hex => print_hex(result),
        OutputFormat::Raw => print_bytes(result)?,
        OutputFormat::HexDump => todo!("Implement hex dump output here"),
    }

    Ok(())
}

fn print_hex(data: Vec<u8>) {
    println!("{}", hex::encode(data))
}

fn print_bytes(data: Vec<u8>) -> Result<()> {
    std::io::stdout().write_all(data.as_slice())?;
    Ok(())
}

fn read_data(read: &mut dyn Read) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    read.read_to_end(&mut data)?;
    Ok(data)
}

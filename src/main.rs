use clap::arg_enum;
use krypt::encode::Encoding;
use krypt::hash::{hash_with_algorithm, HashAlgorithm};
use std::path::PathBuf;
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

fn main() {
    let _m = Options::from_args();
}

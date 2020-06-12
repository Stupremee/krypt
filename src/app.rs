use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "krypt")]
pub struct Options {
    /// The input file for the operation.
    ///
    /// If there's no input file provided, krypt will read from stdin.
    #[structopt(short, long, parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// The output file.
    ///
    /// If no output is provided, krypt will write the output to stdout.
    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// The output format. [hex, raw]
    ///
    /// The hex option will output a hex representation of the data.
    /// The raw option will output the raw bytes of the data.
    #[structopt(short = "f", long = "format", possible_values = &OutputFormat::variants(), default_value = "Raw", case_insensitive = true)]
    pub output_format: OutputFormat,
}

arg_enum! {
    /// The output format
    #[derive(Debug)]
    pub enum OutputFormat {
        Raw,
        Hex,
    }
}

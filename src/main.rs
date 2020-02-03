use clap::arg_enum;
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
enum Mode {}

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

fn main() {
    let _m = Options::from_args();
}

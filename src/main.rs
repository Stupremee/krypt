use clap::{arg_enum, Shell};
use krypt::{
    encode::{self, Encoding},
    hash::{self, HashAlgorithm},
    input::ChunkRead,
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
    /// Generate completion scripts for various shells
    Completions(CompletionsMode),
}

#[derive(StructOpt, Debug)]
struct Options {
    /// Input file. If no input file is provided krypt will read from stdin.
    #[structopt(name = "FILE", short = "i", long = "input", parse(from_os_str))]
    input: Option<PathBuf>,
    /// Specifies if the input is hex encoded.
    #[structopt(long = "hex-input")]
    hex_input: bool,
    /// Specifies the output format.
    #[structopt(short = "f", long = "format", possible_values = &OutputFormat::variants(), default_value = "Raw", case_insensitive = true)]
    output_format: OutputFormat,
    /// Which operation should be executed.
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(StructOpt, Debug)]
struct CompletionsMode {
    #[structopt(name = "SHELL", possible_values = &Shell::variants(), case_insensitive = true)]
    shell: Shell,
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
    match opts.mode {
        Mode::Completions(c) => {
            output_completions_scripts(c.shell);
            std::process::exit(0);
        }
        _ => {}
    };

    let mut file;
    let mut stdin;
    let input: &mut dyn Read = if let Some(path) = opts.input {
        file = File::open(path)?;
        &mut file
    } else {
        stdin = std::io::stdin();
        &mut stdin
    };
    let mut data = if opts.hex_input {
        todo!("implement hex data with the new api")
    } else {
        ChunkRead::new(input)
    };

    let result = match opts.mode {
        Mode::Encode(e) => {
            if e.decode {
                /* if data.last().map_or(false, |e| e == &0x0au8) {
                 *     data.pop();
                 * } */
                encode::decode_data(e.base, data)?
            } else {
                encode::encode_data(e.base, data)
            }
        }
        Mode::Hash(h) => hash::hash_with_algorithm(h.algorithm, &mut data)?,
        Mode::Completions(_) => unreachable!(),
    };

    match opts.output_format {
        OutputFormat::Hex => print_hex(result),
        OutputFormat::Raw => print_bytes(result)?,
        OutputFormat::HexDump => print_hexdump(result)?,
    }

    Ok(())
}

fn print_hex(data: Vec<u8>) {
    println!("{}", hex::encode(data))
}

fn print_bytes(data: Vec<u8>) -> Result<()> {
    std::io::stdout()
        .write_all(data.as_slice())
        .map_err(|e| e.into())
}

fn print_hexdump(data: Vec<u8>) -> Result<()> {
    let mut writer = std::io::stdout();
    let mut p = hexyl::Printer::new(&mut writer, true, hexyl::BorderStyle::Unicode, true);
    p.print_all(data.as_slice(), None)
}

/* fn read_hex_data(read: &mut dyn Read) -> Result<Vec<u8>> {
 *     let mut data = read_data(read)?;
 *     if data.starts_with(&['0' as u8, 'x' as u8]) {
 *         data.remove(0);
 *         data.remove(0);
 *     }
 *     if data.last().map_or(false, |e| e == &0x0au8) {
 *         data.pop();
 *     }
 *     hex::decode(data).map_err(|e| e.into())
 * } */

fn output_completions_scripts(shell: Shell) {
    let mut app = Options::clap();
    let mut writer = std::io::stdout();
    app.gen_completions_to("krypt", shell, &mut writer);
}

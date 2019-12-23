#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate hex;

mod app;
mod encode;
mod hash;
mod log;

use app::{Mode, Options};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let options = app::compile_arguments();
    if options.is_none() {
        return;
    }
    let options = options.unwrap();

    let input = match get_input(&options) {
        Ok(i) => i,
        Err(e) => {
            error!("failed to read input: {}", e);
            return;
        }
    };

    match options.subcommand {
        Mode::Hashing(hash) => execute_hash(hash, input),
        Mode::Encode(encode) => execute_encode(encode, input),
    };
}

fn execute_hash(h: app::Hashing, data: Vec<u8>) {
    let hasher = hash::find_hash_for_name(h.hash.as_str());
    if let Some(hasher) = hasher {
        let result = hasher.hash(data);
        if h.raw_output {
            print_bytes(result);
        } else {
            print!("{}", hex::encode(result));
        }
    } else {
        error!(
            "Invalid hash algorithm provided. Valid algorithms are: {}",
            crate::hash::ALGORITHMS
                .iter()
                .map(|s| s.0)
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }
}

fn execute_encode(e: app::Encode, data: Vec<u8>) {
    let encoder = encode::find_encoder_for_name(e.base.as_str());
    if let Some(encoder) = encoder {
        if e.decode {
            let result = encoder.decode(data);
            if let Err(err) = result {
                error!("Failed to decode input data. {}", err);
                return;
            } else {
                print_bytes(result.unwrap());
            }
        } else {
            let result = encoder.encode(data);
            print!("{}", result);
        }
    } else {
        error!(
            "Invalid base provided. Valid bases are: {}",
            crate::encode::ENCODINGS
                .iter()
                .map(|s| s.0)
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }
}

fn print_bytes(bytes: Vec<u8>) {
    std::io::stdout()
        .write_all(bytes.as_slice())
        .expect("Failed to write to stdout.");
}

fn get_input(options: &Options) -> Result<Vec<u8>, std::io::Error> {
    match &options.input {
        Some(inp) => {
            let mut file = File::open(inp)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            Ok(contents)
        }
        None => {
            let mut contents = Vec::new();
            io::stdin().read_to_end(&mut contents)?;
            Ok(contents)
        }
    }
}

#[macro_use]
extern crate clap;
extern crate ansi_term;

mod app;
mod hash;
mod log;

use app::Options;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let options = app::compile_arguments();
    let input = match get_input(&options) {
        Ok(i) => i,
        Err(e) => {
            error!("failed to read input: {}", e);
            return;
        }
    };
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

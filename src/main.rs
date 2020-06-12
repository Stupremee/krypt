#![warn(rust_2018_idioms)]

mod app;
mod log;

use app::Options;
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

    Ok(())
}

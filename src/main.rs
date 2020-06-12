#![warn(rust_2018_idioms)]

mod app;

use app::Options;
use log::{error, info};
use structopt::StructOpt;

fn main() {
    match try_main() {
        Ok(()) => {}
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable ansi support if target is windows.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    let opt = Options::from_args();

    loggerv::Logger::new()
        .verbosity(opt.verbose.into())
        .level(true)
        .line_numbers(opt.debug)
        .module_path(true)
        .colors(!opt.no_color)
        .init()
        .unwrap();

    Ok(())
}

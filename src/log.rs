//! Logging macros

use std::boxed::Box;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[macro_export]
macro_rules! ok {
    ($title:expr, $msg:expr) => {
        $crate::log::print($title, $msg, termcolor::Color::Green).unwrap()
    };
    ($title:expr, $msg:expr, $($arg:tt)*) => {
        ok!($title, format!($msg, $($arg)*).as_str())
    };
}

#[macro_export]
macro_rules! info {
    ($title:expr, $msg:expr) => {
        $crate::log::print($title, $msg, termcolor::Color::Cyan).unwrap()
    };
    ($title:expr, $msg:expr, $($arg:tt)*) => {
        info!($title, format!($msg, $($arg)*).as_str())
    };
}

#[macro_export]
macro_rules! error {
    ($title:expr, $msg:expr) => {
        $crate::log::print($title, $msg, termcolor::Color::Red).unwrap()
    };
    ($title:expr, $msg:expr, $($arg:tt)*) => {
        error!($title, format!($msg, $($arg)*).as_str())
    };
}

pub(crate) fn print(
    title: &str,
    msg: &str,
    color: Color,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();

    stdout.set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)))?;

    write!(stdout, "{:>12}", title)?;

    stdout.reset()?;
    writeln!(stdout, " {}", msg)?;
    stdout.flush()?;

    Ok(())
}

#[macro_export]
macro_rules! log {
    ($level:ident, $msg:expr, $($args:tt)*) => {
        let prefix = log!(@prefix, $level);
        let message = ansi_term::Style::new().bold().paint(format!($msg, $($args)*));
        println!("{}{}", prefix, message);
    };

    ($level:ident, $msg:expr) => {
        let prefix = log!(@prefix, $level);
        let message = ansi_term::Style::new().bold().paint($msg);
        println!("{}{}", prefix, message);
    };

    (@prefix, warn) => {
        ansi_term::Colour::Yellow.bold().paint("warning: ")
    };

    (@prefix, error) => {
        ansi_term::Colour::Red.bold().paint("error: ")
    };

    (@prefix, debug) => {
        ansi_term::Colour::Purple.bold().paint("debug: ")
    };

    (@prefix, info) => {
        ansi_term::Colour::Blue.bold().paint("info: ")
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr, $($args:tt)*) => {
        log!(warn, $msg, $($args)*)
    };

    ($msg:expr) => {
        log!(warn, $msg)
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr, $($args:tt)*) => {
        log!(error, $msg, $($args)*)
    };

    ($msg:expr) => {
        log!(error, $msg)
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr, $($args:tt)*) => {
        log!(debug, $msg, $($args)*)
    };

    ($msg:expr) => {
        log!(debug, $msg)
    };
}

#[macro_export]
macro_rules! info {
    ($msg:expr, $($args:tt)*) => {
        log!(info, $msg, $($args)*)
    };

    ($msg:expr) => {
        log!(info, $msg)
    };
}

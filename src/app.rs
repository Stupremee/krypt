use clap::{app_from_crate, App, Arg};

#[derive(Debug, Clone)]
pub struct Options {
    pub input: Option<String>,
    pub subcommand: Mode,
}

#[derive(Debug, Clone)]
pub enum Mode {
    Hashing(Hashing),
}

#[derive(Debug, Clone)]
pub struct Hashing {
    pub hash: String,
}

pub fn compile_arguments() -> Options {
    let matches = app_from_crate!()
        .arg(Arg::from_usage(
            "-i, --input=[FILE] 'Sets the input of the operation.'",
        ))
        .subcommand(
            App::new("hash")
                .about("Peforms hash operations.")
                .arg(Arg::from_usage("<hash> 'Sets the hash algorithm to use'")),
        )
        .get_matches();

    let subcommand = match matches.subcommand() {
        ("hash", Some(sub)) => {
            let hash = sub.value_of("hash").map(|s| s.to_owned()).unwrap();
            Mode::Hashing(Hashing { hash })
        }
        _ => unreachable!(),
    };

    Options {
        input: matches.value_of("input").map(|s| s.to_owned()),
        subcommand,
    }
}

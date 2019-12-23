use clap::{app_from_crate, Arg, SubCommand};

#[derive(Debug, Clone)]
pub struct Options {
    pub input: Option<String>,
    pub subcommand: Mode,
}

#[derive(Debug, Clone)]
pub enum Mode {
    Hashing(Hashing),
    Encode(Encode),
}

#[derive(Debug, Clone)]
pub struct Hashing {
    pub hash: String,
    pub raw_output: bool,
}

#[derive(Debug, Clone)]
pub struct Encode {
    pub base: String,
    pub decode: bool,
}

pub fn compile_arguments() -> Option<Options> {
    let matches = app_from_crate!()
        .arg(Arg::from_usage(
            "-i, --input=[FILE] 'Sets the input of the operation.'",
        ))
        .subcommand(
            SubCommand::with_name("hash")
                .about("Peforms hash operations.")
                .arg(Arg::from_usage(
                    "-r, --raw 'Print the result as raw bytes.'",
                ))
                .arg(Arg::from_usage("<hash> 'Sets the hash algorithm to use'")),
        )
        .subcommand(
            SubCommand::with_name("encode")
                .about("Peforms data encoding and decoding.")
                .arg(Arg::from_usage("-d, --decode 'Decode the input data.'"))
                .arg(Arg::from_usage("<base> '16,hex,32,32hex,64,64url'")),
        )
        .get_matches();

    let subcommand = match matches.subcommand() {
        ("hash", Some(sub)) => {
            let hash = sub.value_of("hash").map(|s| s.to_owned()).unwrap();
            let raw_output = sub.is_present("raw");
            Mode::Hashing(Hashing { hash, raw_output })
        }
        ("encode", Some(sub)) => {
            let base = sub.value_of("base").map(|s| s.to_owned()).unwrap();
            let decode = sub.is_present("decode");
            Mode::Encode(Encode { base, decode })
        }
        _ => {
            crate::error!("Please provide a valid operation.");
            return None;
        }
    };

    Some(Options {
        input: matches.value_of("input").map(|s| s.to_owned()),
        subcommand,
    })
}

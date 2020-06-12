use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "krypt")]
pub struct Options {
    /// Adds line numbers to logging output.
    #[structopt(short, long)]
    pub debug: bool,

    /// Verbose mode (-v -vv -vvv).
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Disables colors in log.
    #[structopt(short, long)]
    pub no_color: bool,
}

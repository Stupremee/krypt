#[macro_use]
extern crate clap;
extern crate ansi_term;

mod app;
mod log;

fn main() {
    let options = app::compile_arguments();
    println!("{:?}", options);
}

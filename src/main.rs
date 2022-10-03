use clap::{Parser};

mod args;

use crate::args::args::Arguments;

fn main() {
    let arguments = Arguments::parse();
    println!("Hello, world!");
}

use std::io;

use anyhow::Result;
use clap::Parser;
use json;

/// Command line JSON parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to make request to
    query: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input = io::read_to_string(io::stdin())?;
    println!("");
    println!("{}", args.query);

    let data = json::parse(&input).unwrap();

    println!("{:#}", data);

    Ok(())
}

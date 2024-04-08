use anyhow::{anyhow, Result};
use clap::Parser;

use json::JsonObject;

mod json;

/// Command line JSON parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// JSON to parse
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Ok(parsed) = JsonObject::from_str(&args.input) {
        Ok(())
    } else {
        Err(anyhow!("Could not parse `{}` as json.", args.input))
    }
}

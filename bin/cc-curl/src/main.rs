use std::{fmt, fmt::Display};

use anyhow::{Result, Context};
use clap::Parser;
use url::Url;

#[derive(Debug)]
enum Method {
    GET,
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{self:?}")
    }
}

/// Program that makes http requests
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to make request to
    url: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let url = Url::parse(&args.url).context(format!("Error parsing url `{}`.", args.url))?;

    dbg!(&url);
    
    let host = url.host().ok_or(anyhow::anyhow!("url must include host"))?;
    let method = Method::GET;

    println!("connecting to {host}");
    println!("Sending request {method} {} {}/1.1", url.path(), url.scheme());

    println!("Host: {host}");
    println!("Accept: */*");

    Ok(())
}

use std::{
    fmt, 
    fmt::Display,
    net::TcpStream, io::{Read, Write},
};

use anyhow::{Result, Context};
use clap::Parser;
use url::Url;

const PROTOCOL_VERSION: &str = "1.1";

#[derive(Debug)]
enum Method {
    GET,
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
struct HttpRequest<'a> {
    method: Method,
    url: &'a Url,
    host: &'a str
}

impl<'a> HttpRequest<'a> {
    fn new(method: Method, url: &'a Url) -> Result<Self> {
        let host = url.host_str().ok_or(anyhow::anyhow!("url must include host"))?;
        
        Ok(Self {
            method,
            url,
            host,
        })
    }
    
}

impl<'a> Display for HttpRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}/{PROTOCOL_VERSION}\r\n", self.method, self.url.path(), self.url.scheme().to_uppercase())?;
        write!(f, "Host: {}\r\n", self.host)?;
        write!(f, "Accept: */*\r\n")?;
        write!(f, "Connection: close\r\n")?;
        write!(f, "\r\n")?;

        Ok(())
    }

}

/// Program that makes http requests
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to make request to
    url: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let url = Url::parse(&args.url).context(format!("Error parsing url `{}`.", args.url))?;

    // dbg!(&url);
    
    let request = HttpRequest::new(Method::GET, &url)?;
    let mut response = String::new();

    println!("connecting to {}", request.host);

    let mut stream = TcpStream::connect(format!("{}:80", request.host))?;
    // let mut stream = TcpStream::connect("127.0.0.1:1234")?;

    println!("Sending request:");
    println!("{request}");

    stream.write_all(&format!("{request}").as_bytes())?;
    
    println!();

    stream.read_to_string(&mut response)?;

    println!("{response}");

    Ok(())
}

use std::{
    fmt, 
    fmt::Display,
    net::TcpStream, io::{Read, Write},
};

use anyhow::{Result, Context};
use clap::Parser;
use url::Url;

const PROTOCOL_VERSION: &str = "1.1";
const CRLF: &str = "\r\n";

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
        write!(f, "{} {} {}/{PROTOCOL_VERSION}{CRLF}", self.method, self.url.path(), self.url.scheme().to_uppercase())?;
        write!(f, "Host: {}{CRLF}", self.host)?;
        write!(f, "Accept: */*{CRLF}")?;
        write!(f, "Connection: close{CRLF}")?;
        write!(f, "{CRLF}")?;

        Ok(())
    }

}

/// Program that makes http requests
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to make request to
    url: String,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let url = Url::parse(&args.url).context(format!("Error parsing url `{}`.", args.url))?;

    // dbg!(&url);
    
    let request = HttpRequest::new(Method::GET, &url)?;
    let mut response_string = String::new();

    if args.verbose {
        println!("connecting to {}", request.host);
    }

    let mut stream = TcpStream::connect(format!("{}:80", request.host))?;
    // let mut stream = TcpStream::connect("127.0.0.1:1234")?;

    if args.verbose {
        println!("Sending request:");
    }
    let request_string = format!("{request}");
    stream.write_all(&request_string.as_bytes())?;
    if args.verbose {
        print_http_request(&request_string);
    }
    
    stream.read_to_string(&mut response_string)?;
    let parse_result = parse_response(&response_string);
    match parse_result {
        Ok((response_header, response_body)) => {
            if args.verbose {
                print_http_response(&response_header);
            }
            println!("{response_body}");
        } Err(e) => {
            println!("Error parsing response \n{response_string}\n{e}");
        }
    }

    Ok(())
}

fn print_http_request(request_string: &str) {
    for line in request_string.lines() {
        println!("> {line}");
    }
}

fn print_http_response(response_string: &str) {
    for line in response_string.lines() {
        println!("< {line}");
    }
}

fn parse_response(response_string: &str) -> Result<(&str, &str)> {
    let mut body_start_idx = None;
    
    let blank_line = [0x0d, 0x0a, 0x0d, 0x0a]; // 2 CRLF as hex
    let mut i = 0;
    let mut j = 3;
    let rs = response_string.as_bytes();
    while j < rs.len() {
        if rs[i..=j] == blank_line {
            body_start_idx = Some(j+1);
            break;
        }
        i += 1;
        j += 1;
    }

    if let Some(body_start_idx) = body_start_idx {
        Ok((&response_string[..body_start_idx], &response_string[body_start_idx..]))
    } else {
        Err(anyhow::anyhow!("Error parsing response, could not find blank line to indicate where the message body begins."))
    }
}

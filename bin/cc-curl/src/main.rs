use std::{
    net::TcpStream, io::{Read, Write},
};

use anyhow::{Result, Context};
use clap::Parser;
use url::Url;

mod request;
use request::{HttpRequest, Method};

/// Program that makes http requests
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to make request to
    url: String,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Request method to use.
    /// curl passes on the verbatim string you give it its the request without any filter or other safe guards. That includes white space and control characters.
    #[arg(short = 'X', long)]
    request: Option<String>,

    /// Extra header to include in information sent. When used within an HTTP request, it is added to the regular request headers.
    ///
    /// You  may  specify  any  number  of extra headers. Note that if you should add a custom header that has the same name as one of the internal ones curl
    /// would use, your externally set header is used instead of the internal one. This allows you to make even trickier stuff than curl would  normally  do.
    /// You  should  not  replace internally set headers without knowing perfectly well what you are doing. Remove an internal header by giving a replacement
    /// without content on the right side of the colon, as in: -H "Host:". If you send the custom header with no-value then its  header  must  be  terminated
    /// with a semicolon, such as \-H "X-Custom-Header;" to send "X-Custom-Header:".
    ///
    /// curl  makes  sure  that  each header you add/replace is sent with the proper end-of-line marker, you should thus not add that as a part of the header
    /// content: do not add newlines or carriage returns, they only mess things up for you. curl passes on the verbatim string you give it without any filter
    /// or other safe guards. That includes white space and control characters.
    #[arg(short = 'H', long)]
    header: Option<Vec<String>>,

    /// JSON formatted data to send in the request body
    #[arg(short, long)]
    data: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let url = Url::parse(&args.url).context(format!("Error parsing url `{}`.", args.url))?;

    let method = {
        if let Some(ref request_method) = args.request {
            Method::parse(&request_method)
        } else {
            Method::GET
        }
    };

    let mut request = HttpRequest::new(method, &url)?;
    
    if let Some(ref headers) = args.header {
        for header_string in headers {
            request.parse_header(&header_string);
        }
    }

    if let Some(ref data) = args.data {
        for data_string in data {
            request.add_data(&data_string);
        }
    }

    let mut response_string = String::new();

    if args.verbose {
        println!("connecting to {}", request.host());
    }

    let mut stream = TcpStream::connect(format!("{}:80", request.host()))?;
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

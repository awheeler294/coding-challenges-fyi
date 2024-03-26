use std::{
    collections::HashMap,
    fmt, 
    fmt::Display, ops::{Deref, DerefMut},
};

use anyhow::Result;
use url::Url;

const PROTOCOL_VERSION: &str = "1.1";
const CRLF: &str = "\r\n";

#[derive(Debug)]
pub enum Method<'a> {
    GET,
    POST,
    PUT,
    DELETE,
    Custom(&'a str),
}

impl<'a> Display for Method<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<'a> Method<'a> {
    pub fn parse(s: &'a str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,

            _ => Self::Custom(s),

        }
    }
}

#[derive(Debug)]
pub struct HttpRequest<'a> {
    method: Method<'a>,
    url: &'a Url,
    headers: Headers<'a>,
    body: String,
}

impl<'a> HttpRequest<'a> {
    pub fn new(method: Method<'a>, url: &'a Url) -> Result<Self> {
        let host = url.host_str().ok_or(anyhow::anyhow!("url must include host"))?;
        let mut headers = Headers::new();
        headers.insert("Host", host);
        headers.insert("Accept", "*/*");
        headers.insert("Connection", "close");
        
        Ok(Self {
            method,
            url,
            headers,
            body: String::new(),
        })
    }

    pub fn host(&self) -> &'a str {
        &self.url.host_str().expect("Invariant: url host MUST be validated when the url is set")
    }

    pub fn add_header(&mut self, key: &'a str, value: &'a str) {
        self.headers.insert(key, value);
    }

    pub fn parse_header(&mut self, header_string: &'a str) {
        let header_string = header_string.trim();
        if let Some((key, value)) = header_string.split_once(':') {
            self.add_header(key, value);
            return;
        }

        if header_string.ends_with(';') {
            self.add_header(header_string.trim_end_matches(';'), "");
            return
        }
    }

    pub fn add_data(&mut self, data: &'a str) {
        self.body += data;
    }
    
}

impl<'a> Display for HttpRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}/{PROTOCOL_VERSION}{CRLF}", self.method, self.url.path(), self.url.scheme().to_uppercase())?;
        write!(f, "{}", self.headers)?;
        if self.body.is_empty() == false {
            write!(f, "Content-Length: {}{CRLF}", self.body.len())?;
        }
        write!(f, "{CRLF}")?;
        if self.body.is_empty() == false {
            write!(f, "{}{CRLF}", self.body)?;
        }

        Ok(())
    }

}

#[derive(Debug)]
struct Headers<'a>(HashMap<&'a str, &'a str>);

impl<'a> Deref for Headers<'a> {
    type Target = HashMap<&'a str, &'a str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Headers<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Display for Headers<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in self.iter() {
            write!(f, "{key}: {value}{CRLF}")?;
        }

        Ok(())
    }

}

impl<'a> Headers<'a> {
    fn new() -> Self {
        Self {
            0: HashMap::new()
        }
    }
}

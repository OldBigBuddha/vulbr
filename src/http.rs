use std::string::String;
use std::vec::Vec;

use crate::url::ParsedUrl;

#[derive(Debug)]
pub enum Method {
    Get,
}

impl Method {
    fn name(&self) -> String {
        match self {
            Method::Get => String::from("GET"),
        }
    }
}

#[derive(Debug)]
struct Header {
    key: String,
    value: String,
}

impl Header {
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    method: Method,
    path: String,
    version: String,
    headers: Vec<Header>,
    body: String,
}

impl HttpRequest {
    pub fn new(method: Method, url: &ParsedUrl) -> Self {
        let mut req = Self {
            method,
            path: String::from(&url.path),
            version: String::from("HTTP/1.1"),
            headers: Vec::new(),
            body: String::from("sending a request"),
        };

        req.add_header(String::from("Host"), String::from(&url.host));

        req
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.push(Header::new(key, value));
    }

    pub fn string(&self) -> String {
        // request line
        let mut request = self.method.name();
        request.push(' ');
        request.push_str(&self.path);
        request.push(' ');
        request.push_str(&self.version);
        request.push('\n');

        // headers
        for h in &self.headers {
            request.push_str(&h.key);
            request.push_str(": ");
            request.push_str(&h.value);
            request.push('\n');
        }
        request.push('\n');

        // body
        request.push_str(&self.body);

        request
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    version: String,
    status_code: u8,
    reason: String,
    headers: String,
    body: String,
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Self {
        let preprocessed_response = raw_response.replace("\n\r", "\n");

        let (status_line, remaining) = match preprocessed_response.split_once("\n") {
            Some((s, r)) => (s, r),
            None => panic!("http response doesn't have a new line"),
        };

        let (headers, body) = match remaining.split_once("\n\n") {
            Some((h, b)) => (h, b),
            None => ("", remaining),
        };

        let statuses: Vec<&str> = status_line.split(" ").collect();

        Self {
            version: statuses[0].to_string(),
            status_code: statuses[1].parse().expect("failed to parse status code"),
            reason: statuses[2].to_string(),
            headers: headers.to_string(),
            body: body.to_string(),
        }
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }
}

use std::{fmt,str};
use std::collections::HashMap;

pub type Headers = HashMap<String, Vec<String>>;

pub struct Response {
    code: u32,
    hdrs: Headers,
    body: Vec<u8>,
    total_time: f64,
    primary_ip: Vec<u8>,
    connect_time: f64,
    redirect_count: u32
}

impl Response {
    pub fn new(code: u32, hdrs: Headers, body: Vec<u8>, total_time: f64, primary_ip: Vec<u8>, connect_time: f64, redirect_count: u32) -> Response {
        Response {
            code: code,
            hdrs: hdrs,
            body: body,
	    total_time: total_time,
	    primary_ip: primary_ip,
	    connect_time: connect_time,
	    redirect_count: redirect_count
        }
    }

    pub fn get_total_time(&self) -> f64 {
        self.total_time
    }

    pub fn get_primary_ip<'a>(&'a self) -> &'a [u8] {
        &self.primary_ip
    }

    pub fn get_connect_time(&self) -> f64 {
        self.connect_time
    }

    pub fn get_redirect_count(&self) -> u32 {
        self.redirect_count
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    pub fn get_headers<'a>(&'a self) -> &'a Headers {
        &self.hdrs
    }

    pub fn get_header<'a>(&'a self, name: &str) -> &'a [String] {
        self.hdrs
            .get(name)
            .map(|v| &v[..])
            .unwrap_or(&[])
    }

    pub fn get_body<'a>(&'a self) -> &'a [u8] {
        &self.body
    }

    pub fn move_body(self) -> Vec<u8> {
        self.body
    }
}

impl fmt::Display for Response {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "Response {{{}, ", self.code));

        for (name, val) in self.hdrs.iter() {
            try!(write!(fmt, "{}: {}, ", name, val.connect(", ")));
        }

        match str::from_utf8(&self.body) {
            Ok(b) => try!(write!(fmt, "{}", b)),
            Err(..) => try!(write!(fmt, "bytes[{}]", self.body.len()))
        }

        try!(write!(fmt, "]"));

        Ok(())
    }
}

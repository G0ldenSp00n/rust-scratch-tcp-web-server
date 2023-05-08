use std::{io::Write, net::TcpStream};

pub enum Content<'a> {
    HTML(&'a str),
    JSON(&'a str),
    None,
}

impl<'a> Content<'a> {
    fn length(&self) -> usize {
        match self {
            Content::HTML(content) => content.len(),
            Content::JSON(content) => content.len(),
            Content::None => 0,
        }
    }

    fn data(&self) -> Option<&str> {
        match self {
            Content::JSON(content) => Some(content),
            Content::HTML(content) => Some(content),
            Content::None => None,
        }
    }

    fn content_type(&self) -> Option<&str> {
        match self {
            Content::JSON(_) => Some("application/json"),
            Content::HTML(_) => Some("text/html"),
            Content::None => None,
        }
    }
}

pub enum ResponseCode {
    Success,
    BadRequest,
    ServerError,
}

impl ResponseCode {
    fn to_str(&self) -> &str {
        match self {
            ResponseCode::Success => "200 Ok",
            ResponseCode::BadRequest => "400 Bad Request",
            ResponseCode::ServerError => "500 Server Error",
        }
    }
}

pub struct Result<'a> {
    content: Content<'a>,
    response_code: ResponseCode,
    write_stream: &'a mut TcpStream,
}

impl<'a> Result<'a> {
    pub fn new(
        content: Content<'a>,
        response_code: ResponseCode,
        write_stream: &'a mut TcpStream,
    ) -> Self {
        Self {
            content,
            response_code,
            write_stream,
        }
    }

    pub fn set_content(&mut self, content: Content<'a>) {
        self.content = content;
    }

    pub fn to_respone(&mut self) -> String {
        format!("HTTP/1.1 {}\nServer: hayden-rust\nContent-Type: {}; charset=utf-8\nContent-Length: {}\nConnection: close\n\n{}", self.response_code.to_str(), self.content.content_type().unwrap().to_string(), self.content.length(), self.content.data().unwrap().to_string())
    }

    pub fn send(&mut self) {
        let response = self.to_respone();
        self.write_stream
            .by_ref()
            .write_fmt(format_args!("{}", response))
            .unwrap();
    }
}

#[derive(Debug)]
pub enum Method<'a> {
    GET(&'a str),
}

impl<'a> Method<'a> {
    fn from_raw_http_line(raw_http_line: &'a str) -> Option<Self> {
        let mut line_iter = raw_http_line.split(" ").take(2);
        let identifier = line_iter.next().unwrap_or("");
        let route = line_iter.next().unwrap_or("/");
        match identifier {
            "GET" => Some(Self::GET(route)),
            _ => None,
        }
    }

    pub fn url(&self) -> String {
        match self {
            Self::GET(url) => url.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method<'a>,
    pub content: String,
}

impl<'a> Request<'a> {
    pub fn build_from_raw_http_request(http_request: &'a String) -> Option<Self> {
        if let Some(method_line) = http_request.lines().next() {
            if let Some(method) = Method::from_raw_http_line(&method_line) {
                return Some(Self {
                    method,
                    content: "".to_string(),
                });
            }
        }
        None
    }
}

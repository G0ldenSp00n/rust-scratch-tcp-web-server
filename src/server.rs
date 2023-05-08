use std::{
    collections::HashMap,
    io::{self, Read},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    request::Request,
    result::{Content, ResponseCode, Result},
};

pub trait Endpoint: Send + Sync {
    fn execute(&self, request: &mut Request, response: &mut Result);
}

pub struct DefaultEndpoint {}

impl DefaultEndpoint {
    fn new() -> Self {
        Self {}
    }
}

impl Endpoint for DefaultEndpoint {
    fn execute(&self, _request: &mut Request, response: &mut Result) {
        response.send();
    }
}

#[derive(Clone)]
pub struct Server {
    endpoints: Arc<Mutex<HashMap<String, Arc<Mutex<dyn Endpoint + Send>>>>>,
}

impl Server {
    pub fn build() -> Server {
        Server {
            endpoints: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn tcp_socket_listen(self, tcp_listener: TcpListener) -> std::io::Result<()> {
        for stream in tcp_listener.incoming() {
            let mut stream = stream.unwrap();
            let server = self.clone();
            thread::spawn(move || {
                let mut requests = vec![String::new()];

                loop {
                    let len = requests.len();
                    let mut currently_building_request = requests.get_mut(len - 1).unwrap();
                    io::Read::by_ref(&mut stream)
                        .take(1)
                        .read_to_string(&mut currently_building_request)
                        .unwrap();

                    if currently_building_request
                        .chars()
                        .rev()
                        .take(4)
                        .filter(|char| *char == '\n' || *char == '\r')
                        .collect::<Vec<char>>()
                        .len()
                        == 4
                    {
                        let mut request =
                            Request::build_from_raw_http_request(currently_building_request)
                                .unwrap();
                        println!("{:?}", request);

                        let html = include_str!("main.html");
                        let html = Content::HTML(&html);
                        let mut result = Result::new(html, ResponseCode::Success, &mut stream);

                        server.run_handler(&mut request, &mut result);
                        stream.shutdown(std::net::Shutdown::Both).unwrap();

                        requests.push(String::new());
                    }
                }
            });
        }
        Ok(())
    }

    pub fn register(&mut self, url: String, endpoint: Arc<Mutex<dyn Endpoint + Send>>) {
        println!("Registered URL - {}", url);
        self.endpoints.lock().unwrap().insert(url, endpoint);
    }

    fn run_handler(&self, request: &mut Request, response: &mut Result) {
        if let Some(endpoint) = self.endpoints.lock().unwrap().get(&request.method.url()) {
            endpoint.lock().unwrap().execute(request, response);
        } else {
            DefaultEndpoint::new().execute(request, response);
        }
    }

    pub fn listen(self) -> std::io::Result<()> {
        let ip = "127.0.0.1:8080";
        let listener = TcpListener::bind(ip)?;
        println!("Listening on http://{ip}");

        self.tcp_socket_listen(listener)
    }
}

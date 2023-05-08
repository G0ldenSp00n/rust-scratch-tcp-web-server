mod request;
mod result;
mod server;

use std::sync::{Arc, Mutex};

use server::Endpoint;

use crate::server::Server;

fn main() -> std::io::Result<()> {
    let mut server = Server::build();
    server.register("/json".to_string(), Arc::new(Mutex::new(Home {})));
    server.listen()?;
    Ok(())
}

struct Home {}

impl Endpoint for Home {
    fn execute(&self, _request: &mut request::Request, response: &mut result::Result) {
        response.set_content(result::Content::JSON("{\"test\": 1234}"));
        response.send();
    }
}

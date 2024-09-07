use std::{
    collections::HashMap,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

pub struct HttDee {
    listener: TcpListener,
    port: u16,
}

impl HttDee {
    pub fn new(port: u16, req_handlers: RequestHandlers) -> io::Result<HttDee> {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let listener = TcpListener::bind(socket)?;

        Ok(HttDee { listener, port })
    }

    pub fn start(&self) {
        println!("Server is now listening on port: {}..", self.port);

        for stream in self.listener.incoming() {
            // todo: maybe handle errors later
            let stream = stream.unwrap();

            parse_request(stream);
        }
    }
}

type HandlerClosure = dyn FnOnce(&str);

type Handler = Box<HandlerClosure>;

#[derive(Eq, PartialEq, Hash)]
enum HandlerMethods {
    Get(&'static str),
    Post(&'static str),
}

pub struct RequestHandlers {
    handlers: HashMap<HandlerMethods, Handler>,
}

impl RequestHandlers {
    pub fn new() -> RequestHandlers {
        let handlers = HashMap::new();

        RequestHandlers { handlers }
    }

    pub fn get(&mut self, uri: &'static str, handler: Handler) {
        self.handlers.insert(HandlerMethods::Get(uri), handler);
    }

    pub fn post(&mut self, uri: &'static str, handler: Handler) {
        self.handlers.insert(HandlerMethods::Post(uri), handler);
    }
}

fn parse_request(req: TcpStream) {
    // parse request line from incoming request
    // check for http methods in request line
    todo!("Parse incoming http request")
}

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

pub struct HttDee {
    listener: TcpListener,
    port: u16,
    req_handlers: RequestHandlers,
}

impl HttDee {
    pub fn new(port: u16, req_handlers: RequestHandlers) -> io::Result<HttDee> {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let listener = TcpListener::bind(socket)?;

        Ok(HttDee {
            listener,
            port,
            req_handlers,
        })
    }

    pub fn start(&self) {
        println!("Server is now listening on port: {}..", self.port);

        for stream in self.listener.incoming() {
            // todo: maybe handle errors later
            let stream = stream.unwrap();

            match parse_request(stream) {
                RequestMethods::Get(uri) => {
                    let handler = self
                        .req_handlers
                        .handlers
                        .get(&HandlerMethods::Get(uri.clone()))
                        .unwrap();

                    handler(&uri[..]);
                }
                RequestMethods::Post(uri) => println!("POST URI: {:?}", uri),
                _ => println!("Can't handle this request"),
            }
        }
    }
}

type HandlerClosure = dyn Fn(&str);

type Handler = Box<HandlerClosure>;

#[derive(Eq, PartialEq, Hash)]
enum HandlerMethods {
    Get(String),
    Post(String),
}

pub struct RequestHandlers {
    handlers: HashMap<HandlerMethods, Handler>,
}

impl RequestHandlers {
    pub fn new() -> RequestHandlers {
        let handlers = HashMap::new();

        RequestHandlers { handlers }
    }

    pub fn get<F: Fn(&str) + 'static>(&mut self, uri: &'static str, handler: F) {
        self.handlers
            .insert(HandlerMethods::Get(String::from(uri)), Box::new(handler));
    }

    pub fn post<F: Fn(&str) + 'static>(&mut self, uri: &'static str, handler: F) {
        self.handlers
            .insert(HandlerMethods::Post(String::from(uri)), Box::new(handler));
    }
}

enum RequestMethods {
    Get(String),
    Post(String),
    Other,
}

fn parse_request(mut stream: TcpStream) -> RequestMethods {
    let buf_reader = BufReader::new(&mut stream);

    // "GET / HTTP/1.1"
    let req_line = buf_reader.lines().next().unwrap().unwrap();
    let mut req_line_split = req_line.split(' ');

    let method = req_line_split.next().unwrap().to_owned();
    let uri = req_line_split.next().unwrap().to_owned();

    match &method[..] {
        "GET" => RequestMethods::Get(uri),
        "POST" => RequestMethods::Post(uri),
        _ => RequestMethods::Other,
    }
}

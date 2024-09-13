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
                RequestMethods::Get(uri, _stream) => {
                    let handler = self
                        .req_handlers
                        .handlers
                        .get(&HandlerMethods::Get(uri.clone()))
                        .unwrap_or_else(|| &self.req_handlers.not_found);

                    // let response = handler(&uri[..]);
                    let request = Request {
                        uri,
                        body: String::new(),
                    };

                    let response = handler(request);
                    println!("Response: {}", response);
                }
                RequestMethods::Post(uri, _stream) => println!("POST URI: {:?}", uri),
                _ => println!("HTTP Verb not supported"),
            }
        }
    }
}

#[derive(Debug)]
pub struct Request {
    // pub params   // later,
    pub body: String,
    pub uri: String,
}

type HandlerClosure = dyn Fn(Request) -> String;

type Handler = Box<HandlerClosure>;

#[derive(Eq, PartialEq, Hash)]
enum HandlerMethods {
    Get(String),
    Post(String),
}

pub struct RequestHandlers {
    handlers: HashMap<HandlerMethods, Handler>,
    not_found: Handler,
}

impl RequestHandlers {
    pub fn new() -> RequestHandlers {
        let handlers = HashMap::new();
        let not_found: Handler =
            Box::new(|req| {
                println!("404: Not-Found. Route handler for {} undefined", req.uri);

                format!("404: Not-Found. Route handler for {} undefined", req.uri)
            });

        RequestHandlers {
            handlers,
            not_found,
        }
    }

    pub fn get<F: Fn(Request) -> String + 'static>(&mut self, uri: &'static str, handler: F) {
        self.handlers
            .insert(HandlerMethods::Get(String::from(uri)), Box::new(handler));
    }

    pub fn post<F: Fn(Request) -> String + 'static>(&mut self, uri: &'static str, handler: F) {
        self.handlers
            .insert(HandlerMethods::Post(String::from(uri)), Box::new(handler));
    }
}

enum RequestMethods {
    Get(String, TcpStream),
    Post(String, TcpStream),
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
        "GET" => RequestMethods::Get(uri, stream),
        "POST" => RequestMethods::Post(uri, stream),
        _ => RequestMethods::Other,
    }
}

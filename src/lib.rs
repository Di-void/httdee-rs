use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Write, Read},
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
                RequestMethods::Get(uri, mut stream) => {
                    let handler = self
                        .req_handlers
                        .handlers
                        .get(&HandlerMethods::Get(uri.clone()))
                        .unwrap_or_else(|| &self.req_handlers.not_found);

                    let request = Request {
                        uri,
                        body: String::new(),
                    };

                    let (status, resp) = handler(request);
                    let response = format!("HTTP/1.1 {status}\r\n\r\n{resp}");

                    stream.write_all(response.as_bytes()).unwrap();
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

type HandlerClosure = dyn Fn(Request) -> (String, String);

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
                let status = String::from("404 Not-Found");
                println!("404: Not-Found. Route handler for {} undefined", req.uri);

                (status, format!("Route handler for {} undefined", req.uri))
            });

        RequestHandlers {
            handlers,
            not_found,
        }
    }

    pub fn get<F: Fn(Request) -> (String, String) + 'static>(&mut self, uri: &'static str, handler: F) {
        self.handlers
            .insert(HandlerMethods::Get(String::from(uri)), Box::new(handler));
    }

    pub fn post<F: Fn(Request) -> (String, String) + 'static>(&mut self, uri: &'static str, handler: F) {
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
    let mut buf_reader = BufReader::new(&mut stream);
    let mut headers = String::new();

    // parse headers
    loop {
        let mut line = String::new();
        let n_bytes = buf_reader.read_line(&mut line).unwrap();

        if n_bytes == 0 {
            break;
        }

        if line == "\r\n" {
            break;
        }

        headers.push_str(&line);
    }

    let content_length = parse_content_length(&headers);

    let mut body = vec![0; content_length]; 

    if content_length > 0 {
        buf_reader.read_exact(&mut body).expect("Something went wrong :(");
    }

    let body = String::from_utf8(body).unwrap();

    println!("Request Body: {}", body);

    // "GET / HTTP/1.1"
    let req_line = headers.lines().next().unwrap();
    let method_uri = req_line.split(' ').take(2).collect::<Vec<_>>();
    let [method, uri] = method_uri[..].try_into().expect("req moving mad :(");

    match method {
        "GET" => RequestMethods::Get(uri.to_string(), stream),
        "POST" => RequestMethods::Post(uri.to_string(), stream),
        _ => RequestMethods::Other,
    }
}

fn parse_content_length(headers: &String) -> usize {

    for line in headers.lines() {
        if line.to_lowercase().starts_with("content-length:") {
            let length = line["content-length:".len()..].trim().parse().unwrap();
            return length;
        }
    }

    0
}
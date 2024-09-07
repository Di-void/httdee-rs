use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

pub struct HttDee {
    listener: TcpListener,
    port: u16,
}

impl HttDee {
    pub fn new(port: u16) -> io::Result<HttDee> {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let listener = TcpListener::bind(socket)?;

        Ok(HttDee { listener, port })
    }

    pub fn start(&self) {
        println!("Server is now listening on port: {}", self.port);

        for stream in self.listener.incoming() {
            // todo: maybe handle errors later
            let stream = stream.unwrap();

            parse_request(stream);
        }
    }

    // request handlers
    pub fn get<F>(&self, handler: F)
    where
        F: FnOnce(),
    {
        todo!("do GET work..")
    }

    pub fn post<F>(&self, handler: F)
    where
        F: FnOnce(),
    {
        todo!("do POST work..")
    }
}

fn parse_request(req: TcpStream) {
    // parse request line from incoming request
    // check for http methods in request line
    todo!("Parse incoming http request")
}

use httpdee_rs::{HttDee, RequestHandlers};

fn main() {
    // define handlers
    let mut handlers = RequestHandlers::new();

    // GET request handlers..
    handlers.get("/", |req| println!("Do GET work: {:?}", req));

    // POST request handlers..
    handlers.post("/", |req| println!("Do POST work: {:?}", req));

    // start server
    match HttDee::new(8080, handlers) {
        Ok(server) => {
            // start server
            server.start();
        }
        _ => println!("Failed to bind Socket :("),
    }
}

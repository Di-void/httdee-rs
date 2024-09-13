use httpdee_rs::{HttDee, RequestHandlers};

fn main() {
    // define handlers
    let mut handlers = RequestHandlers::new();

    // GET request handlers..
    handlers.get("/", |req| {
        println!("Do GET work: {:?}", req);

        String::from("Do GET work")
    });

    // POST request handlers..
    handlers.post("/", |req| {
        println!("Do POST work: {:?}", req);

        String::from("Do POST work")
    });

    // start server
    match HttDee::new(8080, handlers) {
        Ok(server) => {
            // start server
            server.start();
        }
        _ => println!("Failed to bind Socket :("),
    }
}

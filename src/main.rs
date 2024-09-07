use httpdee_rs::{HttDee, RequestHandlers};

fn main() {
    // define handlers
    let mut handlers = RequestHandlers::new();

    // GET request handlers..
    handlers.get(
        "/",
        Box::new(|_req| {
            todo!("do GET work..");
        }),
    );

    // POST request handlers..
    handlers.post(
        "/",
        Box::new(|_req| {
            todo!("do POST work..");
        }),
    );

    // start server
    match HttDee::new(8080, handlers) {
        Ok(server) => {
            // start server
            server.start();
        }
        _ => println!("Failed to bind Socket :("),
    }
}

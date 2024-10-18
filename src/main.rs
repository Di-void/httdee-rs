use httpdee_rs::{HttDee, RequestHandlers};

fn main() {
    // define handlers
    let mut handlers = RequestHandlers::new();

    // GET request handlers..
    handlers.get("/", |ctx| {
        let status = String::from("200 OK");
        println!("GET Request Context: {:?}", ctx);

        (status, String::from("GET work"))
    });

    // POST request handlers..
    handlers.post("/", |ctx| {
        let status = String::from("200 OK");
        println!("Do POST work: {:?}", ctx);

        (status, String::from("POST work"))
    });

    // start server
    match HttDee::new(7878, handlers) {
        Ok(server) => {
            // start server
            server.start();
        }
        _ => println!("Failed to bind Socket :("),
    }
}

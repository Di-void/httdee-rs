use httpdee_rs::HttDee;

fn main() {
    // start server
    match HttDee::new(8080) {
        Ok(server) => {
            // start server
            server.start();

            // GET request handler
            server.get(|| {
                // process request
            });

            // POST request handler
            server.post(|| {
                // process request
            })
        }
        _ => println!("Failed to bind Socket :("),
    }
}

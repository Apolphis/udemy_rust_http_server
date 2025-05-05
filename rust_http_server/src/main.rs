fn main() {
    /* let server = Server::new("127.0.0.1:8080");
    server.run(); */
}

struct Server{
    ip_address: String,
}

impl Server{
    fn new(ip_address: String) -> Self {
        Self {
            ip_address
        }
    }

    fn run(self){

    }
}

fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server{
    ip_address: String,
}

struct Request{
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl Server{
    fn new(ip_address: String) -> Self {
        Self {
            ip_address
        }
    }

    fn run(self){
        println!("server is listening on {}", self.ip_address);
    }
}

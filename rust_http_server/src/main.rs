fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server{
    pub struct Server{
        ip_address: String,
    }

    impl Server{
        pub fn new(ip_address: String) -> Self {
            Self {
                ip_address
            }
        }
    
        pub fn run(self){
            println!("server is listening on {}", self.ip_address);
        }
    }

}

mod http {
    mod request{
        struct Request{
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }
    mod method {
        pub enum Method {
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
    }
    
}





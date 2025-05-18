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
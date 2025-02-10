pub mod db_net {
    use crate::db;
    use astra::{Body, Response, ResponseBuilder, Server};
    pub fn start_server(addr: &str) -> Result<(), std::io::Error> {
        Server::bind(addr).serve(|_req, info| {
            ResponseBuilder::new()
                .header("Content-Type", "text/html")
                .body(Body::new("<p>Home page of SigmaDB exposed api</p>"))
                .unwrap()
        })
        
    }


}
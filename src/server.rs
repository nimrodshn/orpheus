use std::convert::Infallible;
use std::net::SocketAddr;
use std::io;
use std::path::Path;
use std::net::{Ipv4Addr, IpAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use crate::memtable::Memtable;

pub struct APIServer{
    port: u16,
    memtable: Memtable
}

impl APIServer {
    pub fn new(port: u16, path: String) -> Result<APIServer, io::Error> {
        let path = Path::new(&path);
        let memtable = Memtable::new(path)?;

        let server = APIServer {
            port,
            memtable
        };
        Ok(server)
    }
    pub async fn run(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        println!("Listening on {}", addr);

        let serve_future = Server::bind(&addr).serve(make_service_fn(|_conn| {
            async {
                {
                    Ok::<_, hyper::Error>(service_fn(serve_req))
                }
            }
        }));

        // Wait for the server to complete serving or exit with an error.
        // If an error occurred, print it to stderr.
        if let Err(e) = serve_future.await {
            eprintln!("server error: {}", e);
        }
    }
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    Ok(Response::new(Body::from("hello, world!")))
}
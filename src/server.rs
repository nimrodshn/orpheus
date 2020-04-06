use std::net::SocketAddr;
use hyper::{Body, Request, Response, Method, StatusCode,Server};
use std::sync::{RwLock, Arc};
use hyper::service::{make_service_fn, service_fn};
use serde::{Deserialize};

use crate::memtable::Memtable;
use crate::handlers;


#[derive(Deserialize, Debug)]
pub struct KeyValuePair {
    pub key : String,
    pub value : String,
}

pub async fn run(port: u16, memtable: Arc<RwLock<Memtable>>) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Listening on {}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(move |_| {
            let cloned_memtable = memtable.clone();
            async move { 
                Ok::<_, hyper::Error>(service_fn(move |req| router(cloned_memtable.clone() ,req))) 
            }
        })
    );

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

// Router routes requests to approporiate http handlers.
async fn router(memtable: Arc<RwLock<Memtable>>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/{key}") => {
            handlers::get_value(memtable, req).await
        }
        (&Method::POST, "/") => {
            handlers::write_key_value_pair(memtable, req).await
        }
        _ => {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}
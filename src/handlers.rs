use hyper::{Body, Request, Response, StatusCode};
use crate::server::KeyValuePair;

pub async fn write_key_value_pair(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let raw = hyper::body::to_bytes(req.into_body()).await?;
    let key_val_pair = match serde_json::from_slice::<KeyValuePair>(raw.as_ref()) {
        Ok(kv) => kv,
        Err(_) => {
            let mut bad_request = Response::default();
            *bad_request.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(bad_request);
        }
    };

    let mut success =  Response::default();
    *success.status_mut() = StatusCode::OK;
    Ok(success)
}

pub fn get_value(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::empty()))
}
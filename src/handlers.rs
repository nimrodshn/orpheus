use hyper::{Body, Request, Response, StatusCode};
use std::sync::{Arc, RwLock};

use crate::memtable::Memtable;
use crate::server::{GetValueRequest, KeyValuePair};

pub async fn write_key_value_pair(
    guarded_memtable: Arc<RwLock<Memtable>>,
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let raw = hyper::body::to_bytes(req.into_body()).await?;
    let key_val_pair = match serde_json::from_slice::<KeyValuePair>(raw.as_ref()) {
        Ok(kv) => kv,
        Err(_) => {
            let mut bad_request = Response::default();
            *bad_request.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(bad_request);
        }
    };

    let mut memtable = match guarded_memtable.write() {
        Ok(rwlock) => rwlock,
        Err(_) => {
            let mut failure = Response::default();
            *failure.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(failure);
        }
    };

    match memtable.write(key_val_pair.key, key_val_pair.value) {
        Ok(_) => {
            let mut success = Response::default();
            *success.status_mut() = StatusCode::OK;
            return Ok(success);
        }
        Err(_) => {
            let mut failure = Response::default();
            *failure.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(failure);
        }
    }
}

pub async fn get_value(
    guarded_memtable: Arc<RwLock<Memtable>>,
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let raw = hyper::body::to_bytes(req.into_body()).await?;
    let get_val_req = match serde_json::from_slice::<GetValueRequest>(raw.as_ref()) {
        Ok(get_val_req) => get_val_req,
        Err(_) => {
            let mut bad_request = Response::default();
            *bad_request.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(bad_request);
        }
    };

    let memtable = match guarded_memtable.read() {
        Ok(memtable) => memtable,
        Err(_) => {
            let mut failure = Response::default();
            *failure.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(failure);
        }
    };

    // read lock requires read method of memtable be immutable! (not &self mut)
    // but read method requires &self mut to access the log file which needs to have
    // mutable reference.
    let value = match memtable.read(&get_val_req.key) {
        Ok(val) => val,
        Err(_) => {
            let mut failure = Response::default();
            *failure.status_mut() = StatusCode::NOT_FOUND;
            return Ok(failure);
        }
    };

    let response = KeyValuePair {
        key: get_val_req.key,
        value: value,
    };

    // write key value response here.
    let raw_response = match serde_json::to_vec::<KeyValuePair>(&response) {
        Ok(raw_response) => raw_response,
        Err(_) => {
            let mut failure = Response::default();
            *failure.status_mut() = StatusCode::NOT_FOUND;
            return Ok(failure);
        }
    };

    let body = Body::from(raw_response);

    let response = match Response::builder().body::<Body>(body) {
        Ok(response) => response,
        Err(_) => {
            let mut failure = Response::default();
            *failure.status_mut() = StatusCode::NOT_FOUND;
            return Ok(failure);
        }
    };

    Ok(response)
}

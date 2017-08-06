extern crate hyper;
extern crate futures;
extern crate lmdb;

#[macro_use]
extern crate lazy_static;

use std::path::Path;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode, Chunk};
use hyper::header::Location;
use futures::{Future, Stream};
use lmdb::{Environment, WriteFlags, Transaction};

lazy_static! {
    static ref DATABASE: Environment = {
        let dir_path = Path::new("/home/dino/go_linkdb");
        let db = Environment::new().open(dir_path).unwrap();
        db
    };
}

struct GoLinkServer;

impl Service for GoLinkServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        let db_handle = DATABASE.open_db(None).unwrap();
        let (method, uri, _version, _headers, body) = req.deconstruct();
        let path_str = uri.path().to_owned();
        let path = &path_str[1..];
        let path = path.to_owned();
        match method {
            Method::Get => {
                futures::future::ok(1).and_then( move |_: u32| {
                let txn = DATABASE.begin_ro_txn().unwrap();
                let value = txn.get(db_handle, &path).unwrap();
                let body = String::from_utf8(value.to_vec()).unwrap();
                response.headers_mut().set(Location::new(body));
                response.set_status(StatusCode::Found);
                Ok(response)
                }).boxed()
            },
            Method::Put => {
                body.concat2().and_then(move |body_c: Chunk| {
                    let mut txn = DATABASE.begin_rw_txn().unwrap();
                    txn.put(db_handle, &path, &body_c, WriteFlags::empty()).unwrap();
                    txn.commit().unwrap();
                    Ok(response)
                }).boxed()
            },
            Method::Options => {
                futures::future::ok(1).and_then( move |_: u32| {
                    let txn = DATABASE.begin_ro_txn().unwrap();
                    let value = txn.get(db_handle, &path).unwrap();
                    let body = String::from_utf8(value.to_vec()).unwrap();
                    response.set_body(body);
                    Ok(response)
                }).boxed()
            },
            _ => {
                response.set_status(StatusCode::BadRequest);
                futures::future::ok(response).boxed()
            },
        }
    }
}

fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, move || Ok(GoLinkServer)).unwrap();
    server.run().unwrap();
}

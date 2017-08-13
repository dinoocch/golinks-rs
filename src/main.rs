extern crate hyper;
extern crate futures;
extern crate lmdb;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;
use std::path::Path;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode, Chunk};
use hyper::header::Location;
use futures::{Future, Stream};
use lmdb::{Environment, WriteFlags, Transaction};

lazy_static! {
    static ref DATABASE: Environment = {
        let args: Vec<String> = env::args().collect();
        info!("Initializing Database Environment");
        let dir_path = Path::new(&args[1]);
        let db = Environment::new().open(dir_path).unwrap();
        info!("[Finished] Initializing Database Environment");
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
        let db_handle = DATABASE.open_db(None);
        match db_handle {
            Ok(db_handle) => {
                let (method, uri, _version, _headers, body) = req.deconstruct();
                let path_str = uri.path().to_owned();
                let path = &path_str[1..];
                let path = path.to_owned();
                debug!("Received new request for {}", path);
                match method {
                    Method::Get => {
                        futures::future::ok(1).and_then( move |_: u32| {
                            debug!("Opening Database Transaction (RO)");
                            let txn = DATABASE.begin_ro_txn();
                            match txn {
                                Ok(txn) => {
                                    let mut path_iter = path.split('/');
                                    let value = match path_iter.next() {
                                        Some(val) => {
                                            txn.get(db_handle, &val)
                                        },
                                        None => { Err(lmdb::Error::Invalid) }
                                    };
                                    match value {
                                        Ok(value) => {
                                            let mut body = String::from_utf8(value.to_vec()).unwrap();
                                            debug!("Got value {} for {}", body, path);
                                            loop {
                                                match path_iter.next() {
                                                    Some(arg) => {
                                                        body = body.replacen("{}", &arg, 1);
                                                    },
                                                    None => { break }
                                                };
                                            };
                                            response.headers_mut().set(Location::new(body));
                                            response.set_status(StatusCode::Found);
                                        },
                                        Err(_) => {
                                            error!("Value not found: {}", path);
                                            response.set_status(StatusCode::NotFound);
                                        }
                                    };
                                },
                                Err(_) => {
                                    error!("Trouble opening database ro transaction");
                                    response.set_status(StatusCode::InternalServerError);
                                }
                            };
                            Ok(response)
                        }).boxed()
                    },
                    Method::Put => {
                        body.concat2().and_then(move |body_c: Chunk| {
                            let txn = DATABASE.begin_rw_txn();
                            match txn {
                                Ok(mut txn) => {
                                    let put = txn.put(db_handle, &path, &body_c, WriteFlags::empty());
                                    match put {
                                        Ok(_) => {
                                            let commit = txn.commit();
                                            match commit {
                                                Ok(_) => {
                                                    response.set_status(StatusCode::NoContent);
                                                },
                                                Err(_) => {
                                                    error!("Error commiting transaction. PUT {}", &path);
                                                    response.set_status(StatusCode::InternalServerError);
                                                }
                                            };
                                        },
                                        Err(_) => {
                                            error!("Could not put into database");
                                            response.set_status(StatusCode::InternalServerError);
                                        }
                                    };
                                },
                                Err(_) => {
                                    error!("Trouble opening database rw transaction");
                                    response.set_status(StatusCode::InternalServerError);
                                }
                            };
                            Ok(response)
                        }).boxed()
                    },
                    Method::Delete => {
                        futures::future::ok(1).and_then( move |_: u32| {
                            let txn = DATABASE.begin_rw_txn();
                            match txn {
                                Ok(mut txn) => {
                                    let value = txn.del(db_handle, &path, None);
                                    match value {
                                        Ok(_) => {
                                            match txn.commit() {
                                                Ok(_) => {
                                                    info!("Deleted entry from database.");
                                                    response.set_status(StatusCode::Found);
                                                },
                                                Err(_) => {
                                                    error!("Could not find entry to delete.");
                                                    response.set_status(StatusCode::NotFound);
                                                }
                                            };
                                        },
                                        Err(_) => {
                                            error!("Could not find entry to delete.");
                                            response.set_status(StatusCode::NotFound);
                                        }
                                    };
                                },
                                Err(_) => {
                                    error!("Could not get read/write transaction");
                                    response.set_status(StatusCode::InternalServerError);
                                }
                            };
                            Ok(response)
                        }).boxed()
                    },
                    Method::Options => {
                        futures::future::ok(1).and_then( move |_: u32| {
                            let txn = DATABASE.begin_ro_txn();
                            match txn {
                                Ok(txn) => {
                                    let value = txn.get(db_handle, &path);
                                    match value {
                                        Ok(value) => {
                                            debug!("OPTIONS requested for {}", path);
                                            let body = String::from_utf8(value.to_vec()).unwrap();
                                            response.set_body(body);
                                        },
                                        Err(_) => {
                                            error!("Value not found (OPTIONS)");
                                            response.set_status(StatusCode::NotFound);
                                        }

                                    }
                                },
                                Err(_) => {
                                    error!("Could not open database transaction.");
                                    response.set_status(StatusCode::InternalServerError);
                                }
                            };
                            Ok(response)
                        }).boxed()
                    },
                    _ => {
                        error!("Received bad request.");
                        response.set_status(StatusCode::BadRequest);
                        futures::future::ok(response).boxed()
                    },
                }
            },
            Err(_) => {
                response.set_status(StatusCode::InternalServerError);
                futures::future::ok(response).boxed()
            }
        }
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, move || Ok(GoLinkServer)).unwrap();
    server.run().unwrap();
}

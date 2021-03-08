use super::proxy_call::call;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use super::mongo::get_port;

use hyper::server::conn::AddrStream;
use hyper::http::uri::InvalidUri;
use std::net::IpAddr;

fn debug_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("{:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

async fn handle(client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let url = req.uri().path();
    let split = url.split("/");
    let list: Vec<&str> = split.collect();
    if list.len() >= 1 {
        let id = list[1];
        let port = get_port(id).await;
        let back = format!("http://127.0.0.1:{}", port);
        let res = call(client_ip, &back, req).await;
        match res {
            Ok(response) => { return Ok(response); }
            Err(error) => {
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap());
            }
        }
    }
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap())
}

pub async fn proxy_call() {
    let bind_addr = "127.0.0.1:8888";
    let addr: SocketAddr = bind_addr.parse().expect("Could not parse ip:port.");

    let make_svc = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| handle(remote_addr, req)))
        }
    });

    println!("Running server on {:?}", &addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn hello(_: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Hello, World!"))
}

fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(|| service_fn_ok(hello));
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

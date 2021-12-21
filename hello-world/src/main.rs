use async_trait::async_trait;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, error::Error, net::SocketAddr};
use utils::SomeAsyncTrait;

struct SomeAsyncStruct;

#[async_trait]
impl SomeAsyncTrait for SomeAsyncStruct {
    async fn do_async_stuff(&self) -> Result<String, Box<dyn Error>> {
        Ok("stuff!".into())
    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    println!("stuff = {}", utils::get_stuff());
    let async_struct = SomeAsyncStruct;
    let res = async_struct.do_async_stuff().await.unwrap();
    println!("res = {}", res);

    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{:?}, oh yeah", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

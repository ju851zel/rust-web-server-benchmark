use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
pub async fn start_server(ip: String, port: i32, _dir: Arc<HashMap<String, Vec<u8>>>) {

    let address = format!("{}:{}", ip, port);
    let address: SocketAddr = address.parse().unwrap(); //correct ip format was handled before

    let make_svc = make_service_fn(|_conn| async {
        let function = service_fn(hello_world);
        Ok::<_, Infallible>(function)
    });

    println!("Hyper server listening for incoming requests on {}", address);

    let server = Server::bind(&address).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    //todo make dir accessible here
    let dir :Arc<HashMap<String, String>> = Arc::new(HashMap::new());
    let uri = req.uri();

    let result = match dir.get(uri.path()) {
        Some(value) => (200, value.to_string()),
        None => (404, "The requested ressource was not found".to_string()),
    };
    //todo return 404 Status Code on error
    Ok(Response::new(result.1.into()))
}

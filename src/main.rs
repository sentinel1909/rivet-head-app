// src/main.rs

// dependencies
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Error, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

// handler function to serve up the web files
async fn serve_file(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Error> {
    let path = match req.uri().path() {
        "/" => PathBuf::from("assets/index.html"),
        _ => PathBuf::from(format!("assets{}", req.uri().path())),
    };

    match File::open(path).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).await.unwrap();
            Ok(Response::new(Full::new(Bytes::from(contents))))
        }
        Err(_) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Not Found")))
            .unwrap()),
    }
}

// main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // get an address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    // create a TcpListener and bind it to the address we just created
    let listener = TcpListener::bind(addr).await?;

    // we need a loop to continuously listen for incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(serve_file))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

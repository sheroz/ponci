#![deny(warnings)]

use log;

use tokio::fs::File;
use tokio::runtime::Runtime;

use tokio_util::codec::{BytesCodec, FramedRead};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Result, Server, StatusCode};

static INDEX: &str = "LICENSE";
static NOTFOUND: &[u8] = b"Not Found";

pub fn start_file_server() -> std::thread::JoinHandle<()>{

    log::info!("Starting file server...");

    std::thread::spawn(|| {

        // Create the async runtime
        let rt  = Runtime::new().unwrap();

        // Execute the future, blocking the current thread until completion
        rt.block_on(async {
            start().await;
        });
    })
}

async fn start() {
    // pretty_env_logger::init();

    let addr = "127.0.0.1:1337".parse().unwrap();

    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response_examples)) });

    let server = Server::bind(&addr).serve(make_service);

    log::info!("Listening file server on http://{}", addr);

    if let Err(e) = server.await {
        log::error!("server error: {}", e);
    }
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => simple_file_send(&req, INDEX).await,
        (&Method::GET, "/no_file.html") => {
            // Test what happens when file cannot be be found
            simple_file_send(&req, "this_file_should_not_exist.html").await
        }
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(_req: &Request<Body>, filename: &str) -> Result<Response<Body>> {

    if log::log_enabled!(log::Level::Debug) {
        log::debug!("recevied file request");
    }

    // Serve a file by asynchronously reading it by chunks using tokio-util crate.
    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}
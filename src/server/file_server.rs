use std::net::SocketAddr;
use log;
use tokio::fs::File;
use tokio::runtime::Runtime;
use tokio_util::codec::{BytesCodec, FramedRead};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Result, Server, StatusCode};
use http::HeaderValue;
use crate::utils::config::Config;

static FORBIDDEN: &[u8] = b"Forbidden";
static NOTFOUND: &[u8] = b"Not Found";
static ACCEPT_RANGES_BYTES: &[u8] = b"bytes";

pub fn start_file_server(config: &Config) -> std::thread::JoinHandle<()>{

    assert!(config.file_server.is_some());
    let file_server_config = config.file_server.as_ref().unwrap();
    assert!(!file_server_config.listen_on.is_empty());
    let listen_on = file_server_config.listen_on[0];

    log::info!("Starting file server...");
    std::thread::spawn(move || {
        let async_runtime  = Runtime::new().unwrap();
        async_runtime.block_on(async {
            start(listen_on).await;
        });
    })
}

async fn start(listen_on: SocketAddr) {

    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(file_service)) });

    let server = Server::bind(&listen_on).serve(make_service);

    log::info!("Listening file server on http://{}", listen_on);

    if let Err(e) = server.await {
        log::error!("server error: {}", e);
    }
}

async fn file_service(req: Request<Body>) -> Result<Response<Body>> {
    match req.method() {
        &Method::GET => file_send(&req).await,
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

/// HTTP status code 403
fn forbidden() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(FORBIDDEN.into())
        .unwrap()
}

async fn file_send(req: &Request<Body>) -> Result<Response<Body>> {
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("recevied request:{:#?}", req);
    }

    let filename = req.uri().path().replace("/", "");
    if filename.is_empty() {
        return Ok(forbidden());
    }

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("recevied request, filename:{}", filename);
    }
    
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.
    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        let mut response = Response::new(body);
        let headers = response.headers_mut();
        headers.insert(hyper::header::ACCEPT_RANGES,HeaderValue::from_bytes(ACCEPT_RANGES_BYTES).unwrap());

        if log::log_enabled!(log::Level::Trace) {
            log::trace!("response headers:{:#?}", response.headers());
        }
    
        if log::log_enabled!(log::Level::Trace) {
            log::trace!("response:{:#?}", response);
        }

        return Ok(response);
    }

    Ok(not_found())
}
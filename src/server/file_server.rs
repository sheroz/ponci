use std::net::SocketAddr;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, Result, StatusCode};
use hyper_util::rt::TokioIo;

use bytes::Bytes;
use http::HeaderValue;
use http_body_util::Full;
use crate::utils::http_range;

use log;

use crate::utils::config::Config;

// A simple type alias so as to DRY.
type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn start_file_server(config: &Config, flag_ready: Arc<AtomicBool>, flag_shutdown: Arc<AtomicBool>) -> JoinHandle<()> {
    assert!(config.file_server.is_some());
    let file_server_config = config.file_server.as_ref().unwrap();
    assert!(!file_server_config.listen_on.is_empty());
    let listen_on = file_server_config.listen_on[0];

    log::info!("Starting file server...");

    std::thread::spawn(move || {
        let async_runtime = Runtime::new().unwrap();
        async_runtime.block_on(async {
            if let Err(err) = start(listen_on, flag_ready, flag_shutdown).await {
                log::error!("File server failed: {:?}", err);
            }
        });
    })
}

async fn start(listen_on: SocketAddr, flag_ready: Arc<AtomicBool>, flag_shutdown: Arc<AtomicBool>) -> std::result::Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind(listen_on).await?;

    flag_ready.store(true, Ordering::SeqCst);

    log::info!("File server running on http://{}", listen_on);

    while !flag_shutdown.load(Ordering::SeqCst) {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            let io = TokioIo::new(stream);
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(file_service))
                .await
            {
                log::error!("Failed to serve connection: {:?}", err);
            }
        });
    }
    Ok(())
}

async fn file_service(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {
    match req.method() {
        &Method::HEAD => file_info(&req).await,
        &Method::GET => file_send(&req).await,
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::new()))
        .expect("unable to build 404 response")
}

/// HTTP status code 403
fn forbidden() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Full::new(Bytes::new()))
        .expect("unable to build 403 response")
}

async fn file_info(req: &Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("recevied request:{:#?}", req);
    }

    let filename = req.uri().path().replace("/", "");
    if filename.is_empty() {
        return Ok(forbidden());
    }

    if log::log_enabled!(log::Level::Debug) {
        log::debug!("recevied request {}, filename:{}", req.method(), filename);
    }

    match get_file_len(&filename).await {
        Ok(file_len) => {
            let response = Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::ACCEPT_RANGES, http_range::RANGE_UNIT)
            .header(hyper::header::CONTENT_LENGTH, file_len)
            .body(Full::new(Bytes::new()))
            .expect("unable to build response");
        
            if log::log_enabled!(log::Level::Trace) {
                log::trace!("response:{:#?}", response);
            }
            Ok(response)
        }
        Err(_err) => {
            Ok(not_found())
        }
    }

}

async fn get_file_len(filename: &str) -> MyResult<u64>{
    let file  = tokio::fs::File::open(filename).await?;
    let metadata = file.metadata().await?;
    if metadata.is_file() {
        let file_len = metadata.len();
        if log::log_enabled!(log::Level::Trace) {
            log::trace!("The length of the file '{}' is {} bytes", filename, file_len)
        }
        return Ok(file_len);
    }
    let err_msg = format!("Not a file: {}", filename);
    log::error!("{err_msg}");
    Err(err_msg.into())
}

async fn file_send(req: &Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("recevied request:{:#?}", req);
    }

    let filename = req.uri().path().replace("/", "");
    if filename.is_empty() {
        return Ok(forbidden());
    }

    if log::log_enabled!(log::Level::Debug) {
        log::debug!("recevied request {}, filename:{}", req.method(), filename);
    }

    let mut content_total_len = 0;
    let mut ranges: Vec<Range<u64>> = vec![];
    let headers = req.headers();
    if headers.contains_key(hyper::header::CONTENT_RANGE) {
        let _file_len = get_file_len(&filename).await;
        if let Ok(file_len) = _file_len {
            content_total_len = file_len;            
            let content_range = headers.get(hyper::header::CONTENT_RANGE).unwrap();
            let _ranges = http_range::parse(content_range.to_str().unwrap(), file_len);
            if _ranges.is_some() {
                ranges = _ranges.unwrap();
            }
        } else {
            return Ok(not_found());
        }
    }
    if ranges.is_empty() {
        if let Ok(contents) = tokio::fs::read(&filename).await {
            let body = contents.into();
            let mut response = Response::new(Full::new(body));
    
            let headers = response.headers_mut();
            headers.insert(
                hyper::header::ACCEPT_RANGES,
                HeaderValue::from_str(http_range::RANGE_UNIT).unwrap(),
            );
    
            return Ok(response);
        }
    } else {
        for range in ranges {

            if log::log_enabled!(log::Level::Debug) {
                log::debug!("preparing the range to send {:?}", range);
            }
        
            use tokio::io::AsyncSeekExt;
            use std::io::SeekFrom;
            use bytes::BytesMut;
            let capacity = (range.end - range.start + 1) as usize;
            let mut buffer = BytesMut::with_capacity(capacity);
            if let Ok(mut file) = tokio::fs::File::open(&filename).await {
                if let Ok(_) = file.seek(SeekFrom::Start(range.start)).await {
                    if let Ok(read_count) = file.read_exact(&mut buffer).await {
                        let body: Bytes = buffer.into();
    
                        let response = Response::builder()
                        .status(StatusCode::PARTIAL_CONTENT)
                        .header(hyper::header::ACCEPT_RANGES, http_range::RANGE_UNIT)
                        .header(hyper::header::CONTENT_LENGTH, read_count)
                        .header(hyper::header::CONTENT_RANGE, format!("{} {}-{}/{}",http_range::RANGE_UNIT, range.start, range.end, content_total_len))
                        .body(Full::new(body))
                        .expect("unable to build response");
                
                        return Ok(response);
                    } else {
                        log::error!("could not read bytes");
                    }
                } else {
                    log::error!("could not seek position: {}", range.start);
                }
            } else {
                log::error!("could not open file: {}", filename);
            }
        }        
    }

    Ok(not_found())
}

/*

https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range

Content-Range: <unit> <range-start>-<range-end>/<size>
Content-Range: <unit> <range-start>-<range-end>/*
Content-Range: <unit> */<size>

*/
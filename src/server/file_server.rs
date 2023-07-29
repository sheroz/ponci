use std::io::SeekFrom;
use std::net::SocketAddr;
use std::ops::Range;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncSeekExt;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, Result, StatusCode};
use hyper_util::rt::TokioIo;

use crate::utils::http_range;
use bytes::Bytes;
use http_body_util::Full;

use log;

use crate::utils::config::Config;

// A simple type alias so as to DRY.
type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn start_file_server(
    config: &Config,
    flag_ready: Arc<AtomicBool>,
    flag_shutdown: Arc<AtomicBool>,
) -> JoinHandle<()> {
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

async fn start(
    listen_on: SocketAddr,
    flag_ready: Arc<AtomicBool>,
    flag_shutdown: Arc<AtomicBool>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    blank_response(StatusCode::NOT_FOUND)
}

/// HTTP status code 403
fn forbidden() -> Response<Full<Bytes>> {
    blank_response(StatusCode::FORBIDDEN)
}

/// HTTP status code 500
fn internal_server_error() -> Response<Full<Bytes>> {
    blank_response(StatusCode::INTERNAL_SERVER_ERROR)
}

/// A blank response with status code
fn blank_response(status_code: StatusCode) -> Response<Full<Bytes>> {
    let mut response = Response::<Full<Bytes>>::new(Full::new(Bytes::new()));
    *response.status_mut() = status_code;
    response
}

async fn file_info(req: &Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("recevied request:{:#?}", req);
    }

    let filename = req.uri().path().replace("/", "");
    if filename.is_empty() {
        log::error!("filename is empty");
        return Ok(forbidden());
    }

    if log::log_enabled!(log::Level::Debug) {
        log::debug!("recevied request {}, filename:{}", req.method(), filename);
    }

    match get_file_len(&filename).await {
        Ok(file_len) => {
            if let Ok(response) = Response::builder()
                .status(StatusCode::OK)
                .header(hyper::header::ACCEPT_RANGES, http_range::RANGE_UNIT)
                .header(hyper::header::CONTENT_LENGTH, file_len)
                .body(Full::new(Bytes::new()))
            {
                if log::log_enabled!(log::Level::Trace) {
                    log::trace!("response:{:#?}", response);
                }
                Ok(response)
            } else {
                log::error!("unable to build response");
                Ok(internal_server_error())
            }
        }
        Err(_err) => {
            log::error!("file not found: {}", filename);
            Ok(not_found())
        }
    }
}

async fn get_file_len(filename: &str) -> MyResult<u64> {
    let file = tokio::fs::File::open(filename).await?;
    let metadata = file.metadata().await?;
    if metadata.is_file() {
        let file_len = metadata.len();
        if log::log_enabled!(log::Level::Trace) {
            log::trace!(
                "The length of the file '{}' is {} bytes",
                filename,
                file_len
            )
        }
        return Ok(file_len);
    }
    let err_msg = format!("Not a file: {}", filename);
    log::error!("{err_msg}");
    Err(err_msg.into())
}

async fn file_send(req: &Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {

    let content_type: &str = "text/html; charset=utf-8";

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("recevied request:{:#?}", req);
    }

    let filename = req.uri().path().replace("/", "");
    if filename.is_empty() {
        log::error!("filename is empty");
        return Ok(forbidden());
    }

    if log::log_enabled!(log::Level::Debug) {
        log::debug!("recevied request {}, filename:{}", req.method(), filename);
    }

    let mut content_size = 0;
    let mut ranges: Vec<Range<u64>> = vec![];
    let headers = req.headers();
    if headers.contains_key(hyper::header::CONTENT_RANGE) {
        let _file_len = get_file_len(&filename).await;
        if let Ok(file_len) = _file_len {
            content_size = file_len;
            let content_range = headers.get(hyper::header::CONTENT_RANGE).unwrap();
            let http_range = http_range::parse(content_range.to_str().unwrap(), file_len);
            if http_range.is_some() {
                ranges = http_range.unwrap().ranges;
            }
        } else {
            log::error!("file not found: {}", filename);
            return Ok(not_found());
        }
    }
    if ranges.is_empty() {
        if let Ok(contents) = tokio::fs::read(&filename).await {
            let body = contents.into();
            if let Ok(response) = Response::builder()
                .status(StatusCode::OK)
                .header(hyper::header::ACCEPT_RANGES, http_range::RANGE_UNIT)
                .header(hyper::header::CONTENT_TYPE, content_type)
                .body(Full::new(body))
            {
                return Ok(response);
            } else {
                log::error!("unable to build response");
                return Ok(internal_server_error());
            }
        }
    } else {
        for range in ranges {
            let capacity = (range.end - range.start + 1) as usize;
            if log::log_enabled!(log::Level::Trace) {
                log::trace!("preparing the range to send {:?}", range);
                log::trace!("capacity {}", capacity);
            }

            if range.start > content_size {

                // 416 Range Not Satisfiable
                // https://datatracker.ietf.org/doc/html/rfc7233#section-4.4

                if let Ok(response) = Response::builder()
                    .status(StatusCode::RANGE_NOT_SATISFIABLE)
                    .header(hyper::header::ACCEPT_RANGES, http_range::RANGE_UNIT)
                    .header(
                        hyper::header::CONTENT_RANGE,
                        format!(
                            "{} */{}",
                            http_range::RANGE_UNIT,
                            content_size
                        ),
                    )
                    .body(Full::new(Bytes::new()))
                {
                    if log::log_enabled!(log::Level::Debug) {
                        log::debug!("Range Not Satisfiable (416). Requested range is greater than existing content, {} > {}", range.start, content_size);
                    }
                    return Ok(response);
                } else {
                    log::error!("unable to build response");
                    return Ok(internal_server_error());
                }
            }

            let mut buffer = vec![0; capacity];
            if let Ok(mut file) = tokio::fs::File::open(&filename).await {
                if let Ok(_seek) = file.seek(SeekFrom::Start(range.start)).await {
                    if log::log_enabled!(log::Level::Trace) {
                        log::trace!("seek result {}", _seek);
                    }
                    if let Ok(read_count) = file.read_exact(&mut buffer).await {
                        if log::log_enabled!(log::Level::Trace) {
                            log::trace!("read_count {}", read_count);
                        }
                        let body: Bytes = buffer.into();
                        if let Ok(response) = Response::builder()
                            .status(StatusCode::PARTIAL_CONTENT)
                            .header(hyper::header::ACCEPT_RANGES, http_range::RANGE_UNIT)
                            .header(
                                hyper::header::CONTENT_RANGE,
                                format!(
                                    "{} {}-{}/{}",
                                    http_range::RANGE_UNIT,
                                    range.start,
                                    range.end,
                                    content_size
                                ),
                            )
                            .header(hyper::header::CONTENT_TYPE, content_type)
                            .body(Full::new(body))
                        {
                            return Ok(response);
                        } else {
                            log::error!("unable to build response");
                            return Ok(internal_server_error());
                        }
                    } else {
                        log::error!("could not read bytes from file");
                    }
                } else {
                    log::error!("could not seek file position: {}", range.start);
                }
            } else {
                log::error!("could not open file: {}", filename);
            }
        }
    }

    log::error!("file not found: {}", filename);
    Ok(not_found())
}

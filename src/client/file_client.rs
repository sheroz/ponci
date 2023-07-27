use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::Request;
use tokio::io::{self, AsyncWriteExt as _};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use hyper_util::rt::TokioIo;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn get_file(url: &str) {
    let url = url.parse::<hyper::Uri>().unwrap();

    let async_runtime = Runtime::new().unwrap();
    async_runtime.block_on(async {
        let result = request_url("GET", url).await;
        match result {
            Err(err) =>  {
                log::error!("Connection failed: {:?}", err);
            }
            Ok(()) => ()
        }
    });
}

pub fn get_file_info(url: &str) {
    let url = url.parse::<hyper::Uri>().unwrap();

    let async_runtime = Runtime::new().unwrap();
    async_runtime.block_on(async {
        let result = request_url("HEAD", url).await;
        match result {
            Err(err) =>  {
                log::error!("Connection failed: {:?}", err);
            }
            Ok(()) => ()
        }
    });
}

async fn request_url(method: &str, url: hyper::Uri) -> Result<()> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            log::error!("Connection failed: {:?}", err);
        }
    });

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("File client connected to {}://{}:{}", url.scheme().unwrap(), host, port);
    }

    let authority = url.authority().unwrap().clone();

    let req = Request::builder()
        .uri(url)
        .method(method)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("Request:\n{:#?}", req);
    }

    let mut res = sender.send_request(req).await?;

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("Response: {}", res.status());
        log::trace!("Headers:\n{:#?}", res.headers());
    }

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(&chunk).await?;
        }
    }

    Ok(())
}

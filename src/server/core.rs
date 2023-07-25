use crate::utils::config::Config;
use log;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::thread::{self, JoinHandle};

pub trait TcpServer<'a> {
    fn with_config(config: &'a Config) -> Self;
    fn start(
        &'a self,
        server_shutdown: &Arc<AtomicBool>,
        server_ready: &Arc<AtomicBool>,
    );
    fn stop();
    fn set_item(key: String, item: StorageItem) -> bool;
    fn get_item(key: String) -> Option<StorageItem>;
    fn remove_item(key: String) -> bool;
}

pub struct PoncuTcpServer<'a> {
    _storage: HashMap<String, StorageItem>,
    config: &'a Config,
}

pub type PoncuMutex <'a> = Arc<Mutex<&'a PoncuTcpServer <'a>> >;

pub struct StorageItem {
    _item_type: ItemComplexType,
    _data: Box<Vec<u8>>,
    _description: String,
    _tags: Vec<String>,
    _metadata: HashMap<String, String>,
    _may_expire: bool,
    _expires_on: std::time::Instant,
    _storage: Vec<ItemStorageType>,
    _redundancy: u8, // min number of required replications in the claster: 0,1,2, …
}

pub enum ItemComplexType {
    Array(ItemBasicType),
    Set(ItemBasicType),
    Map(ItemBasicType, ItemBasicType),
    Blob,
    Json,
    Xml,
    File,
    Folder,
    Path,
}

pub enum ItemBasicType {
    String,
    Boolean,
    SignedInteger(u8),
    UnsignedInteger(u8),
    Float(u8),
}

/// TBD
pub enum ItemStorageType {
    Memory, // default
    Disk,
}

impl<'a> TcpServer<'a> for PoncuTcpServer<'a> {
    fn with_config(config: &'a Config) -> Self {
        PoncuTcpServer {
            _storage: HashMap::new(),
            config,
        }
    }

    fn start(&self, shutdown: &Arc<AtomicBool>, ready: &Arc<AtomicBool>) {

        assert!(self.config.server.is_some());
        let config_server = self.config.server.as_ref().unwrap();
        assert!(!config_server.listen_on.is_empty());
        let socket_address = config_server.listen_on[0];

        let listener = TcpListener::bind(socket_address).unwrap();
        ready.store(true, Ordering::SeqCst);

        log::info!("started listening on {} ...", socket_address);
        // listener.set_nonblocking(true).unwrap();

        // using thread pooling
        // Final Project: Building a Multithreaded Web Server
        // https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
        let mut handles = Vec::<JoinHandle<()>>::new();
        while !shutdown.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, addr)) => {
                    let connection_shutdown = shutdown.clone();
                    let handle = thread::spawn(move|| {
                        handle_connection(stream, addr, connection_shutdown)
                    });
                    handles.push(handle);
                },
                /*
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    thread::sleep(time::Duration::from_millis(1));
                    continue;
                }
                */
                Err(e) => log::error!("couldn't get client: {e:?}"),
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }

    }

    fn stop() {}

    fn set_item(_key: String, _item: StorageItem) -> bool {
        false
    }

    fn get_item(_key: String) -> Option<StorageItem> {
        None
    }

    fn remove_item(_key: String) -> bool {
        true
    }
}

fn handle_connection(mut stream: TcpStream, addr: SocketAddr, shutdown: Arc<AtomicBool>) {
    log::debug!("client connected: {}", addr);
/*
    use std::io::BufRead;
    let mut reader = BufReader::new(stream);
    let mut msg = String::new();
    reader.read_line(&mut msg).unwrap();
    log::debug!("received message: {}", msg);

*/
    let mut buf = [0;1024];
    let addr = stream.peer_addr().unwrap();
    while !shutdown.load(Ordering::SeqCst) {
        let count = stream.read(&mut buf).unwrap();
        log::debug!("received bytes count from {} : {}", addr, count);
        let mut vec = buf.to_vec();
        vec.truncate(count);
        let msg = String::from_utf8(vec).unwrap();
        log::debug!("received message from {} : {}", addr, msg);
    }
}

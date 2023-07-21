use crate::utils::config::Config;
use log;
use std::collections::HashMap;
use std::io::BufReader;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub trait TcpServer<'a> {
    fn with_config(config: &'a Config) -> Self;
    fn start(
        &self,
        server_shutdown: Arc<AtomicBool>,
        server_ready: Arc<AtomicBool>,
    ) -> JoinHandle<()>;
    fn stop();
    fn set_item(key: String, item: StorageItem) -> bool;
    fn get_item(key: String) -> Option<StorageItem>;
    fn remove_item(key: String) -> bool;
}

pub struct PoncuTcpServer<'a> {
    _storage: HashMap<String, StorageItem>,
    config: &'a Config,
}

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

    fn start(&self, shutdown: Arc<AtomicBool>, ready: Arc<AtomicBool>) -> JoinHandle<()> {
        assert!(self.config.server.is_some());
        let config_server = self.config.server.as_ref().unwrap();
        assert!(!config_server.listen_on.is_empty());
        let socket_address = config_server.listen_on[0];

        let handle = thread::spawn(move || {
            let listener = TcpListener::bind(socket_address).unwrap();
            ready.store(true, Ordering::SeqCst);

            log::info!("started listening on {} ...", socket_address);
            // listener.set_nonblocking(true).unwrap();

            while !shutdown.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, addr)) => handle_connection(stream, addr, shutdown.clone()),
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
        });

        handle
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

fn handle_connection(stream: TcpStream, addr: SocketAddr, _shutdowm: Arc<AtomicBool>) {
    use std::io::BufRead;
    log::debug!("client connected: {}", addr);

    let mut reader = BufReader::new(stream);
    let mut msg = String::new();
    reader.read_line(&mut msg).unwrap();
    log::debug!("received message: {}", msg);
}

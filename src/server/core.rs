use std::io;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use log;

pub trait TcpServer {
    fn new(ip_address: IpAddr, port: u16) -> Self;
    fn start(&self, server_shutdown: Arc<AtomicBool>, server_ready: Arc<AtomicBool>) -> JoinHandle<()>;
    fn stop();
    fn set_item(key: String, item: StorageItem) -> bool;
    fn get_item(key: String) -> Option<StorageItem>;
    fn remove_item(key: String) -> bool;
}

pub struct PoncuTcpServer {
    storage: HashMap<String, StorageItem>,
    ip_address: IpAddr,
    port: u16
}

pub struct StorageItem {
    item_type: ItemComplexType,
    data: Box<Vec<u8>>,
    description: String,
    tags: Vec<String>,
    metadata: HashMap<String, String>,
    may_expire: bool,
    expires_on: std::time::Instant,
    storage: Vec<ItemStorageType>,
    redundancy: u8 // min number of required replications in the claster: 0,1,2, …
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

impl TcpServer for PoncuTcpServer {
    fn new(ip_address: IpAddr, port: u16) -> Self {
        PoncuTcpServer {
            storage: HashMap::new(),
            ip_address,
            port
        }
    }

    fn start(&self, shutdown: Arc<AtomicBool>, ready: Arc<AtomicBool>) -> JoinHandle<()> {
        let socket_address = SocketAddr::new(self.ip_address, self.port);

        let handle = thread::spawn(move || {

            let listener = TcpListener::bind(socket_address).unwrap();
            ready.store(true, Ordering::SeqCst);

            log::info!("started listening on {}:{} ...", socket_address.ip(), socket_address.port());
            // listener.set_nonblocking(true).unwrap();
            
            while !shutdown.load(Ordering::SeqCst) {
                let connection_close = shutdown.clone();
                match listener.accept() {
                    Ok((stream, addr)) => handle_connection(stream, addr, connection_close),
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

    fn stop() {

    }

    fn set_item(key: String, item: StorageItem) -> bool {
        false
    }

    fn get_item(key: String) -> Option<StorageItem> {
        None
    }

    fn remove_item(key: String) -> bool {
        true
    }
}

fn handle_connection(stream: TcpStream, addr: SocketAddr, connection_close: Arc<AtomicBool>) {
    log::debug!("client connected: {:?}", addr)
}

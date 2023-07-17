use std::net::{IpAddr, SocketAddr, TcpListener};

pub fn start(port:u16, ip_address:IpAddr) {
    let socket_address = SocketAddr::new(ip_address, port);
    let listener = TcpListener::bind(socket_address).unwrap();
    match listener.accept() {
        Ok((_stream, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }
}
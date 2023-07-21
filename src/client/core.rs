use std::io::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream};

pub trait TcpClient {
    fn new(ip_address:IpAddr, port:u16) -> Self;
    fn with_socket(listen_socket: &SocketAddr) -> Self;
    fn connect(&mut self) -> std::io::Result<()>;
    fn disconnect(&mut self)  -> std::io::Result<()>;
    fn set_item(&mut self, key: String) -> std::io::Result<()>;
    fn get_item(&mut self, key: String) -> std::io::Result<()>;
    fn remove_item(&self, key: String) -> bool;
}
pub struct PoncuTcpClient {
    socket_addr: SocketAddr,
    stream: Option<TcpStream>,
}

impl TcpClient for PoncuTcpClient {
    fn new(ip_address:IpAddr, port:u16) -> Self {
        let socket_addr: SocketAddr = SocketAddr::new(ip_address, port);
        PoncuTcpClient::with_socket(&socket_addr)
    }

    fn with_socket(socket_addr: &SocketAddr) -> Self {
        PoncuTcpClient {socket_addr: socket_addr.clone(), stream: None}
    }

    fn connect(&mut self) -> std::io::Result<()> {
        let stream = TcpStream::connect(self.socket_addr)?;

        let local_addr = stream.local_addr().unwrap();
        log::info!("connected to {} as {}", self.socket_addr, local_addr);

        self.stream = Some(stream);
        Ok(())
    }

    fn disconnect(&mut self) -> std::io::Result<()> {
        let stream = self.stream.as_mut().unwrap();
        stream.flush()?;
        self.stream = None;
        Ok(())
    }

    fn set_item(&mut self, key: String) -> std::io::Result<()> {
        use std::io::BufWriter;
        let stream = self.stream.as_mut().unwrap();
        
        let mut writer = BufWriter::new(stream);
        writeln!(writer, "{key}")?;
        Ok(())
    }

    fn get_item(&mut self, key: String) -> std::io::Result<()>  {
        let stream = self.stream.as_mut().unwrap();
        let mut buf = [0; 128];
        stream.write(&buf)?;
        stream.read(&mut buf)?;
        Ok(())
    }

    fn remove_item(&self, key: String) -> bool {
        true
    }
}

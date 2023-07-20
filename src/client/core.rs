use std::io::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream};

pub trait TcpClient {
    fn new(ip_address:IpAddr, port:u16) -> Self;
    fn connect(&mut self) -> std::io::Result<()>;
    fn disconnect(&mut self)  -> std::io::Result<()>;
    fn set_item(&mut self, key: String) -> std::io::Result<()>;
    fn get_item(&mut self, key: String) -> std::io::Result<()>;
    fn remove_item(&self, key: String) -> bool;
}
pub struct PoncuTcpClient {
    port: u16,
    ip_address: IpAddr,
    stream: Option<TcpStream>,
}

impl TcpClient for PoncuTcpClient {
    fn new(ip_address:IpAddr, port:u16) -> Self {
        PoncuTcpClient {port, ip_address, stream: None}
    }

    fn connect(&mut self) -> std::io::Result<()> {
        let socket_address = SocketAddr::new(self.ip_address, self.port);
        let stream = TcpStream::connect(socket_address)?;

        let local_addr = stream.local_addr().unwrap();
        log::info!("connected to {}:{} as {}:{}", self.ip_address, self.port, local_addr.ip(), local_addr.port());

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

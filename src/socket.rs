use std::net::{SocketAddr, UdpSocket};

pub struct ServerSocket {
    socket: UdpSocket,
}

impl ServerSocket {
    pub fn new(addr: &SocketAddr) -> std::io::Result<ServerSocket> {
        let socket = UdpSocket::bind(addr)?;
        Ok(ServerSocket { socket })
    }
}

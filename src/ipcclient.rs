use std::{io::{self, Read, Write}, net::{IpAddr, TcpStream}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    address: u32,
    length: usize,
}

pub struct IpcReader {
    socket: TcpStream,
    address: u32,
}

impl IpcReader {
    pub fn new(ip: IpAddr, port: u16, address: u32) -> io::Result<Self> {
        Ok(IpcReader {
            socket: TcpStream::connect((ip, port))?,
            address,
        })
    }
}

impl Read for IpcReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let length = buf.len();
        let req = Request { address: self.address, length };
        self.address += length as u32;

        let req = serde_json::to_string(&req).unwrap();
        self.socket.write_all(req.as_bytes()).unwrap();

        self.socket.read(buf)
    }
}

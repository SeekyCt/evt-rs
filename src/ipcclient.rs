use std::{io::{self, Read, Write}, net::{IpAddr, TcpStream}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    // address: u32,
    length: usize,
}

pub struct IpcReader {
    socket: TcpStream,
}

impl IpcReader {
    pub fn new(ip: IpAddr, port: u16) -> io::Result<Self> {
        Ok(IpcReader {
            socket: TcpStream::connect((ip, port))?
        })
    }
}

impl Read for IpcReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let req = Request { length: buf.len() };
        let req = serde_json::to_string(&req).unwrap();
        self.socket.write_all(req.as_bytes()).unwrap();

        self.socket.read(buf)
    }
}

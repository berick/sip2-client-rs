use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use log::{info, error};
use super::{Message, Field, FixedField};
use super::error::Error;
use super::spec;
use super::util;

pub struct Client {
    sip_host: String, // E.g. 127.0.0.1:6001
    tcp_stream: TcpStream,
}

impl Client {

    pub fn new(sip_host: &str) -> Result<Self, Error>  {

        match TcpStream::connect(sip_host) {
            Ok(stream) => {
                return Ok(
                    Client {
                        sip_host: sip_host.to_string(),
                        tcp_stream: stream
                    }
                );
            }
            Err(s) => {
                error!("Client::new() failed: {}", s);
                return Err(Error::NetworkError);
            }
        }
    }

    pub fn disconnect(&self) -> Result<(), Error> {

        match self.tcp_stream.shutdown(Shutdown::Both) {
            Ok(_) => Ok(()),
            Err(s) => {
                error!("disconnect() failed: {}", s);
                return Err(Error::NetworkError);
            }
        }
    }

    pub fn send(&mut self, msg: Message) -> Result<(), Error> {
        let msg_sip = msg.to_sip();

        match self.tcp_stream.write(&msg_sip.as_bytes()) {
            Ok(_) => Ok(()),
            Err(s) => {
                error!("send() failed: {}", s);
                return Err(Error::NetworkError);
            }
        }
    }

    pub fn recv(&mut self) -> Result<Message, Error> {
        // TODO
        Message::from_sip("")
    }
}






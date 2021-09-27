use std::str;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use log::{debug, info, error};
use super::{Message, Field, FixedField};
use super::error::Error;
use super::spec;
use super::util;

const READ_BUFSIZE: usize = 256;

pub struct Client {
    sip_host: String, // E.g. 127.0.0.1:6001
    tcp_stream: TcpStream,
}

impl Client {

    /// Creates a new SIP client and opens the TCP connection to the server
    ///
    /// * `sip_host` - SIP server host/ip and port
    ///   ** e.g. "127.0.0.1:6001"
    ///
    /// ```
    /// use sip2::Client;
    /// assert_eq!(Client::new("JUNK0+..-*z$@").is_err(), true);
    /// ```
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

    pub fn send(&mut self, msg: &Message) -> Result<(), Error> {
        let msg_sip = msg.to_sip();

        debug!("OUTBOUND: {}", msg_sip);

        match self.tcp_stream.write(&msg_sip.as_bytes()) {
            Ok(_) => Ok(()),
            Err(s) => {
                error!("send() failed: {}", s);
                return Err(Error::NetworkError);
            }
        }
    }

    pub fn recv(&mut self) -> Result<Message, Error> {

        let mut text = String::from("");

        loop {

            let mut buf: [u8; READ_BUFSIZE] = [0; READ_BUFSIZE];

            let num_bytes = match self.tcp_stream.read(&mut buf) {
                Ok(num) => num,
                Err(s) => {
                    error!("recv() failed: {}", s);
                    return Err(Error::NetworkError);
                }
            };

            if num_bytes == 0 { break; }

            let chunk = match str::from_utf8(&buf) {
                Ok(s) => s,
                Err(s) => {
                    error!("recv() got non-utf data: {}", s);
                    return Err(Error::MessageFormatError);
                }
            };

            text.push_str(chunk);

            if num_bytes < READ_BUFSIZE { break; }
        }

        debug!("INBOUND: {}", text);

        Message::from_sip(&text)
    }
}






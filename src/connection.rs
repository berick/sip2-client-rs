use std::str;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use log::{trace, debug, error};
use super::{Message};
use super::error::Error;
use super::spec;

const READ_BUFSIZE: usize = 256;

pub struct Connection {
    tcp_stream: TcpStream,
}

impl Connection {

    /// Creates a new SIP client and opens the TCP connection to the server
    ///
    /// * `sip_host` - SIP server host/ip and port
    ///   ** e.g. "127.0.0.1:6001"
    ///
    /// ```
    /// use sip2::Connection;
    /// assert_eq!(Connection::new("JUNK0+..-*z$@").is_err(), true);
    /// ```
    pub fn new(sip_host: &str) -> Result<Self, Error>  {
        debug!("Connection::new() connecting to: {}", sip_host);

        match TcpStream::connect(sip_host) {
            Ok(stream) => Ok(Connection { tcp_stream: stream }),
            Err(s) => {
                error!("Connection::new() failed: {}", s);
                return Err(Error::NetworkError);
            }
        }
    }

    /// Shutdown the TCP connection with the SIP server.
    pub fn disconnect(&self) -> Result<(), Error> {
        debug!("Connection::disconnect()");

        match self.tcp_stream.shutdown(Shutdown::Both) {
            Ok(_) => Ok(()),
            Err(s) => {
                error!("disconnect() failed: {}", s);
                return Err(Error::NetworkError);
            }
        }
    }

    /// Send a SIP message
    pub fn send(&mut self, msg: &Message) -> Result<(), Error> {

        let msg_sip = msg.to_sip() + spec::LINE_TERMINATOR;

        debug!("OUTBOUND: {}", msg_sip);

        match self.tcp_stream.write(&msg_sip.as_bytes()) {
            Ok(_) => Ok(()),
            Err(s) => {
                error!("send() failed: {}", s);
                return Err(Error::NetworkError);
            }
        }
    }

    /// Receive a SIP response.
    ///
    /// Blocks until a response is received.
    pub fn recv(&mut self) -> Result<Message, Error> {

        trace!("Connection::recv() waiting for response...");

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

        if text.len() == 0 {
            Err(Error::NoResponseError)
        } else {
            // Discard the line terminator and any junk after it.
            let mut parts = text.split(spec::LINE_TERMINATOR);

            match parts.next() {
                Some(s) => Message::from_sip(s),
                None => Err(Error::MessageFormatError),
            }
        }
    }

    /// Shortcut for self.send() + self.recv().
    pub fn sendrecv(&mut self, msg: &Message) -> Result<Message, Error> {
        self.send(msg)?;
        self.recv()
    }
}


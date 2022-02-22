use std::str;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use log::{trace, debug, error};
use super::{spec, Message, Field, FixedField};
use super::error::Error;
use super::connection::Connection;

const READ_BUFSIZE: usize = 256;

pub struct Client {
    connection: Connection,
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
    pub fn new(host: &str, port: u32) -> Result<Self, Error>  {
        let uri = String::from(host) + ":" + &(port.to_string());
        let mut con = Connection::new(&uri)?;
        Ok(Client { connection: con })
    }

    pub fn disconnect(&self) -> Result<(), Error> {
        self.connection.disconnect()
    }

    // TODO location code
    pub fn login(&mut self, username: &str, password: &str) -> Result<(), Error> {

        let login = Message::new(
            &spec::M_LOGIN,
            vec![
                FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
                FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
            ],
            vec![
                Field::new(spec::F_LOGIN_UID.code, &username),
                Field::new(spec::F_LOGIN_PWD.code, &password),
            ],
        );

        let resp = self.connection.sendrecv(&login)?;

        if resp.spec().code == spec::M_LOGIN_RESP.code
            && resp.fixed_fields().len() == 1
            && resp.fixed_fields()[0].value() == "1" {

            return Ok(());
        }

        return Err(Error::LoginError);
    }
}



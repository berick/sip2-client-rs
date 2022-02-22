use std::str;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use log::{trace, debug, error};
use super::{spec, Message, Field, FixedField, util};
use super::error::Error;
use super::connection::Connection;

pub struct PatronStatusParams {
    pub patron_barcode: String,
    pub patron_pwd: Option<String>,
    pub institution: Option<String>,
    pub terminal_pwd: Option<String>,
}

impl PatronStatusParams {
    pub fn new(patron_barcode: &str) -> Self {
        PatronStatusParams {
            patron_barcode: patron_barcode.to_string(),
            patron_pwd: None,
            institution: None,
            terminal_pwd: None,
        }
    }
}

pub struct SipResponse {
    msg: Message,
    ok: bool,
}

impl SipResponse {
    pub fn ok(&self) -> bool {
        self.ok
    }

    pub fn msg(&self) -> &Message {
        &self.msg
    }
}

/// Wrapper for Connection which provides a simpler interface for some
/// common SIP2 actions.

pub struct Client {
    connection: Connection,
}

impl Client {

    /// Creates a new SIP client and opens the TCP connection to the server.
    pub fn new(host: &str, port: u32) -> Result<Self, Error>  {
        let uri = String::from(host) + ":" + &(port.to_string());
        let mut con = Connection::new(&uri)?;
        Ok(Client { connection: con })
    }

    pub fn disconnect(&self) -> Result<(), Error> {
        self.connection.disconnect()
    }

    /// Login to the SIP server
    ///
    /// Sets ok=true if the OK fixed field is true.
    pub fn login(&mut self,
        username: Option<&str>,
        password: Option<&str>,
        location: Option<&str>) -> Result<SipResponse, Error> {

        let mut req = Message::new(
            &spec::M_LOGIN,
            vec![
                FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
                FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
            ],
            vec![]
        );

        req.maybe_add_field(spec::F_LOGIN_UID.code, username);
        req.maybe_add_field(spec::F_LOGIN_PWD.code, password);
        req.maybe_add_field(spec::F_LOCATION_CODE.code, location);

        let resp = self.connection.sendrecv(&req)?;

        if resp.spec().code == spec::M_LOGIN_RESP.code
            && resp.fixed_fields().len() == 1
            && resp.fixed_fields()[0].value() == "1" {

            Ok(SipResponse {ok: true, msg: resp})

        } else {

            Ok(SipResponse {ok: false, msg: resp})
        }
    }

    /// Send the SC status message
    ///
    /// Sets ok=true if the server reports that it's online.
    pub fn sc_status(&mut self) -> Result<SipResponse, Error> {

        let req = Message::new(
            &spec::M_SC_STATUS,
            vec![
                FixedField::new(&spec::FF_STATUS_CODE, "0").unwrap(),
                FixedField::new(&spec::FF_MAX_PRINT_WIDTH, "999").unwrap(),
                FixedField::new(
                    &spec::FF_PROTOCOL_VERSION, &spec::SIP_PROTOCOL_VERSION
                ).unwrap(),
            ],
            vec![],
        );

        let resp = self.connection.sendrecv(&req)?;

        if resp.fixed_fields().len() > 0
            && resp.fixed_fields()[0].value() == "Y" {

            Ok(SipResponse {ok: true, msg: resp})

        } else {

            Ok(SipResponse {ok: false, msg: resp})
        }
    }

    pub fn patron_status(&mut self, params: &PatronStatusParams) -> Result<SipResponse, Error> {
        let mut req = Message::new(
            &spec::M_PATRON_STATUS,
            vec![
                FixedField::new(&spec::FF_LANGUAGE, "000").unwrap(),
                FixedField::new(&spec::FF_DATE, &util::sip_date_now()).unwrap(),
            ],
            vec![Field::new(spec::F_PATRON_ID.code, &params.patron_barcode)],
        );

        req.maybe_add_field(spec::F_INSTITUTION_ID.code, params.institution.as_deref());
        req.maybe_add_field(spec::F_PATRON_PWD.code, params.patron_pwd.as_deref());
        req.maybe_add_field(spec::F_TERMINAL_PWD.code, params.terminal_pwd.as_deref());

        let resp = self.connection.sendrecv(&req)?;

        if let Some(bl_val) = resp.get_field_value(spec::F_VALID_PATRON.code) {
            if bl_val == "Y" {
                return Ok(SipResponse {ok: true, msg: resp});
            }
        }

        Ok(SipResponse {ok: false, msg: resp})
    }
}



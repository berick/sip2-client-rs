use std::str;
use super::error::Error;
use super::connection::Connection;
use super::{spec, Message, Field, FixedField, util};

/// A set of structs for capturing sets of required and optional
/// SIP request parameters.

pub struct LoginParams {
    pub username: String,
    pub password: String,
    pub location: Option<String>,
}

impl LoginParams {
    pub fn new(username: &str, password: &str) -> Self {
        LoginParams {
            username: username.to_string(),
            password: password.to_string(),
            location: None,
        }
    }
}

pub struct ItemInfoParams {
    pub item_id: String,
    pub institution: Option<String>,
    pub terminal_pwd: Option<String>,
}

impl ItemInfoParams {
    pub fn new(item_id: &str) -> Self {
        ItemInfoParams {
            item_id: item_id.to_string(),
            institution: None,
            terminal_pwd: None,
        }
    }
}

pub struct PatronStatusParams {
    pub patron_id: String,
    pub patron_pwd: Option<String>,
    pub institution: Option<String>,
    pub terminal_pwd: Option<String>,
}

impl PatronStatusParams {
    pub fn new(patron_id: &str) -> Self {
        PatronStatusParams {
            patron_id: patron_id.to_string(),
            patron_pwd: None,
            institution: None,
            terminal_pwd: None,
        }
    }
}

pub struct PatronInfoParams {
    pub patron_id: String,
    pub patron_pwd: Option<String>,

    /// Indicates which position (if any) of the summary string should
    /// be set to 'Y' (i.e. activated).  Only one summary index may
    /// be activated per message.  Positions are zero-based.
    pub summary: Option<usize>,

    pub institution: Option<String>,
    pub terminal_pwd: Option<String>,
    pub start_item: Option<usize>,
    pub end_item: Option<usize>,
}

impl PatronInfoParams {
    pub fn new(patron_id: &str) -> Self {
        PatronInfoParams {
            patron_id: patron_id.to_string(),
            patron_pwd: None,
            summary: None,
            institution: None,
            terminal_pwd: None,
            start_item: None,
            end_item: None,
        }
    }
}

/// Wrapper for holding the SIP response message and a simplistic
/// "OK" flag.
pub struct SipResponse {

    /// The response message.
    msg: Message,

    /// True if the message response indicates a success.
    ///
    /// The definition of success varies per request type and may not
    /// match the caller's requirements.  See the full message in
    /// 'msg' to inspect the entire response.
    ok: bool,
}

impl SipResponse {
    /// Shortcut for this.resp.msg().get_field_value(code)
    pub fn value(&self, code: &str) -> Option<String> {
        self.msg().get_field_value(code)
    }
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
        let con = Connection::new(&uri)?;
        Ok(Client { connection: con })
    }

    pub fn disconnect(&self) -> Result<(), Error> {
        self.connection.disconnect()
    }

    /// Login to the SIP server
    ///
    /// Sets ok=true if the OK fixed field is true.
    pub fn login(&mut self, params: &LoginParams) -> Result<SipResponse, Error> {

        let mut req = Message::new(
            &spec::M_LOGIN,
            vec![
                FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
                FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
            ],
            vec![
                Field::new(spec::F_LOGIN_UID.code, &params.username),
                Field::new(spec::F_LOGIN_PWD.code, &params.password),
            ]
        );

        req.maybe_add_field(spec::F_LOCATION_CODE.code, params.location.as_deref());

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

    /// Send a patron status request
    ///
    /// Sets ok=true if the "valid patron" (BL) field is "Y"
    pub fn patron_status(&mut self, params: &PatronStatusParams) -> Result<SipResponse, Error> {
        let mut req = Message::new(
            &spec::M_PATRON_STATUS,
            vec![
                FixedField::new(&spec::FF_LANGUAGE, "000").unwrap(),
                FixedField::new(&spec::FF_DATE, &util::sip_date_now()).unwrap(),
            ],
            vec![Field::new(spec::F_PATRON_ID.code, &params.patron_id)],
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

    /// Send a patron information request
    ///
    /// Sets ok=true if the "valid patron" (BL) field is "Y"
    pub fn patron_info(&mut self, params: &PatronInfoParams) -> Result<SipResponse, Error> {

        let mut summary: [char; 10] = [' '; 10];

        if let Some(idx) = params.summary {
            if idx < 10 {
                summary[idx] = 'Y';
            }
        }

        let sum_str: String = summary.iter().collect::<String>();

        let mut req = Message::new(
            &spec::M_PATRON_INFO,
            vec![
                FixedField::new(&spec::FF_LANGUAGE, "000").unwrap(),
                FixedField::new(&spec::FF_DATE, &util::sip_date_now()).unwrap(),
                FixedField::new(&spec::FF_SUMMARY, &sum_str).unwrap(),
            ],
            vec![Field::new(spec::F_PATRON_ID.code, &params.patron_id)],
        );

        req.maybe_add_field(spec::F_INSTITUTION_ID.code, params.institution.as_deref());
        req.maybe_add_field(spec::F_PATRON_PWD.code, params.patron_pwd.as_deref());
        req.maybe_add_field(spec::F_TERMINAL_PWD.code, params.terminal_pwd.as_deref());

        if let Some(v) = params.start_item {
            req.add_field(spec::F_START_ITEM.code, &v.to_string());
        }

        if let Some(v) = params.end_item {
            req.add_field(spec::F_END_ITEM.code, &v.to_string());
        }

        let resp = self.connection.sendrecv(&req)?;

        if let Some(bl_val) = resp.get_field_value(spec::F_VALID_PATRON.code) {
            if bl_val == "Y" {
                return Ok(SipResponse {ok: true, msg: resp});
            }
        }

        Ok(SipResponse {ok: false, msg: resp})
    }

    /// Send a item information request
    ///
    /// Sets ok=true if a title (AJ) value is present.  Oddly, there's no
    /// specific "item does not exist" value in the Item Info Response.
    pub fn item_info(&mut self, params: &ItemInfoParams) -> Result<SipResponse, Error> {

        let mut req = Message::new(
            &spec::M_ITEM_INFO,
            vec![
                FixedField::new(&spec::FF_DATE, &util::sip_date_now()).unwrap(),
            ],
            vec![Field::new(spec::F_ITEM_IDENT.code, &params.item_id)],
        );

        req.maybe_add_field(spec::F_INSTITUTION_ID.code, params.institution.as_deref());
        req.maybe_add_field(spec::F_TERMINAL_PWD.code, params.terminal_pwd.as_deref());

        let resp = self.connection.sendrecv(&req)?;

        if let Some(title_val) = resp.get_field_value(spec::F_TITLE_IDENT.code) {
            if title_val != "" {
                return Ok(SipResponse {ok: true, msg: resp});
            }
        }

        Ok(SipResponse {ok: false, msg: resp})
    }
}



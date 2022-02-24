#![allow(dead_code)]
use super::Message;

/// Sructs for capturing collections of required and optional
/// SIP request parameters and a sip response wrapper.


#[derive(Debug, Clone)]
pub struct ParamBuilder {
    institution: Option<String>,
    terminal_pwd: Option<String>,
    sip_user: Option<String>,
    sip_pass: Option<String>,
    location: Option<String>,
    patron_id: Option<String>,
    patron_pwd: Option<String>,
    item_id: Option<String>,
    start_item: Option<usize>,
    end_item: Option<usize>,

    /// Indicates which position (if any) of the summary string should
    /// be set to 'Y' (i.e. activated).  Only one summary index may
    /// be activated per message.  Positions are zero-based.
    summary: Option<usize>,
}

impl ParamBuilder {

    pub fn new() -> Self {
        ParamBuilder {
            institution: None,
            terminal_pwd: None,
            sip_user: None,
            sip_pass: None,
            location: None,
            patron_id: None,
            patron_pwd: None,
            item_id: None,
            start_item: None,
            end_item: None,
            summary: None,
        }
    }

    pub fn set_institution(&mut self, value: &str) -> &mut Self {
        self.institution = Some(value.to_string());
        self
    }
    pub fn set_terminal_pwd(&mut self, value: &str) -> &mut Self {
        self.terminal_pwd = Some(value.to_string());
        self
    }
    pub fn set_sip_user(&mut self, value: &str) -> &mut Self {
        self.sip_user = Some(value.to_string());
        self
    }
    pub fn set_sip_pass(&mut self, value: &str) -> &mut Self {
        self.sip_pass = Some(value.to_string());
        self
    }
    pub fn set_location(&mut self, value: &str) -> &mut Self {
        self.location = Some(value.to_string());
        self
    }
    pub fn set_patron_id(&mut self, value: &str) -> &mut Self {
        self.patron_id = Some(value.to_string());
        self
    }
    pub fn set_patron_pwd(&mut self, value: &str) -> &mut Self {
        self.patron_pwd = Some(value.to_string());
        self
    }
    pub fn set_item_id(&mut self, value: &str) -> &mut Self {
        self.item_id = Some(value.to_string());
        self
    }
    pub fn set_start_item(&mut self, value: usize) -> &mut Self {
        self.start_item = Some(value);
        self
    }
    pub fn set_end_item(&mut self, value: usize) -> &mut Self {
        self.end_item= Some(value);
        self
    }
    pub fn set_summary(&mut self, value: usize) -> &mut Self {
        self.summary = Some(value);
        self
    }

    ///
    ///
    /// Panics if required values are missing.
    pub fn to_login(&self) -> LoginParams {

        LoginParams {
            sip_user: self.sip_user.as_ref().unwrap().to_string(),
            sip_pass: self.sip_pass.as_ref().unwrap().to_string(),
            location: self.location.clone(),
        }
    }
}

pub struct LoginParams {
    sip_user: String,
    sip_pass: String,
    location: Option<String>,
}

impl LoginParams {
    pub fn sip_user(&self) -> &str {
        &self.sip_user
    }

    pub fn sip_pass(&self) -> &str {
        &self.sip_pass
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
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

    pub fn new(msg: Message, ok: bool) -> Self {
        SipResponse {
            msg,
            ok,
        }
    }

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

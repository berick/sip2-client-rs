use super::Message;

/// Sructs for capturing collections of required and optional
/// SIP request parameters and a sip response wrapper.

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

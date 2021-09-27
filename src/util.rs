use std::collections::HashMap;
use chrono::{DateTime, Local};
use std::fmt;
use super::spec;
use super::error;

pub const LINE_TERMINATOR: char = '\r';
pub const SIP_DATE_FORMAT: &str = "%Y%m%d    %H%M%S";
pub const STRING_COLUMN_PAD: u8 = 32; // for printing/debugging

pub struct Util {
    field_map: HashMap<&'static str, &'static spec::Field>,
    message_map: HashMap<&'static str, &'static spec::Message>,
}

impl Util {

    pub fn new() -> Self {
        let mut u = Util {
            field_map: HashMap::new(),
            message_map: HashMap::new(),
        };
        //u.set_field_map();
        u.set_message_map();
        u
    }

    /*
    fn set_field_map(&mut self) {
        self.field_map.insert(spec::F_LOGIN_UID.code, &spec::F_LOGIN_UID);
        self.field_map.insert(spec::FF_DATE.code, &spec::F_
        self.field_map.insert(spec::FF_OK.code, &spec::F_
        self.field_map.insert(spec::FF_UID_ALGO.code, &spec::F_
        self.field_map.insert(spec::FF_PWD_ALGO.code, &spec::F_
        self.field_map.insert(spec::FF_FEE_TYPE.code, &spec::F_
        self.field_map.insert(spec::FF_PAYMENT_TYPE.code, &spec::F_
        self.field_map.insert(spec::FF_CURRENCY.code, &spec::F_
        self.field_map.insert(spec::FF_PAYMENT_ACCEPTED.code, &spec::F_
        self.field_map.insert(spec::FF_CIRCULATION_STATUS.code, &spec::F_
        self.field_map.insert(spec::FF_SECURITY_MARKER.code, &spec::F_
        self.field_map.insert(spec::FF_LANGUAGE.code, &spec::F_
        self.field_map.insert(spec::FF_PATRON_STATUS.code, &spec::F_
        self.field_map.insert(spec::FF_SUMMARY.code, &spec::F_
        self.field_map.insert(spec::FF_HOLD_ITEMS_COUNT.code, &spec::F_
        self.field_map.insert(spec::FF_OD_ITEMS_COUNT.code, &spec::F_
        self.field_map.insert(spec::FF_CH_ITEMS_COUNT.code, &spec::F_
        self.field_map.insert(spec::FF_FINE_ITEMS_COUNT.code, &spec::F_
        self.field_map.insert(spec::FF_RECALL_ITEMS_COUNT.code, &spec::F_
        self.field_map.insert(spec::FF_UNAVAIL_ITEMS_COUNT.code, &spec::F_
        self.field_map.insert(spec::FF_RENEWAL_POLICY.code, &spec::F_
        self.field_map.insert(spec::FF_NO_BLOCK.code, &spec::F_
        self.field_map.insert(spec::FF_NB_DUE_DATE.code, &spec::F_
        self.field_map.insert(spec::FF_STATUS_CODE.code, &spec::F_
        self.field_map.insert(spec::FF_MAX_PRINT_WIDTH.code, &spec::F_
        self.field_map.insert(spec::FF_PROTOCOL_VERSION.code, &spec::F_
        self.field_map.insert(spec::FF_RENEW_OK.code, &spec::F_
        self.field_map.insert(spec::FF_MAGNETIC_MEDIA.code, &spec::F_
        self.field_map.insert(spec::FF_DESENSITIZE.code, &spec::F_
        self.field_map.insert(spec::FF_RESENSITIZE.code, &spec::F_
        self.field_map.insert(spec::FF_RETURN_DATE.code, &spec::F_
        self.field_map.insert(spec::FF_ALERT.code, &spec::F_
        self.field_map.insert(spec::FF_ONLINE_STATUS.code, &spec::F_
        self.field_map.insert(spec::FF_CHECKIN_OK.code, &spec::F_
        self.field_map.insert(spec::FF_CHECKOUT_OK.code, &spec::F_
        self.field_map.insert(spec::FF_ACS_RENEWAL_POLICY.code, &spec::F_
        self.field_map.insert(spec::FF_STATUS_UPDATE_OK.code, &spec::F_
        self.field_map.insert(spec::FF_OFFLINE_OK.code, &spec::F_
        self.field_map.insert(spec::FF_TIMEOUT_PERIOD.code, &spec::F_
        self.field_map.insert(spec::FF_RETRIES_ALLOWED.code, &spec::F_
        self.field_map.insert(spec::FF_DATETIME_SYNC.code, &spec::F_
        self.field_map.insert(spec::FF_THIRD_PARTY_ALLOWED.code, &spec::F_
        self.field_map.insert(spec::FF_RENEWED_COUNT.code, &spec::F_
        self.field_map.insert(spec::FF_UNRENEWED_COUNT.code, &spec::F_
        self.field_map.insert(spec::FF_HOLD_MODE.code, &spec::F_
        self.field_map.insert(spec::FF_HOLD_AVAILABLE.code, &spec::F_
    }
    */

    fn set_message_map(&mut self) {
        self.message_map.insert(spec::M_LOGIN.code, &spec::M_LOGIN);
        self.message_map.insert(spec::M_SC_STATUS.code, &spec::M_SC_STATUS);
    }

    pub fn get_field_by_code(&mut self, code: &str) -> Option<&spec::Field> {
        // HashMap values are static refs.
        // HashMap.get() returns Option<&&Field>.
        // Unpack here with * cuz returning &&Field is weird
        match self.field_map.get(code) {
            Some(f) => Some(*f),
            None => None
        }
    }

    pub fn get_message_by_code(&self, code: &str) -> Option<&spec::Message> {
        match self.message_map.get(code) {
            Some(m) => Some(*m),
            None => None
        }
    }

    /// Clean up a string for inclusion in a SIP message
    ///
    /// ```
    /// use sip2::util::Util;
    /// let result = Util::sip_string("howdy|par|dner");
    /// assert_eq!(result, "howdypardner");
    /// ```
    ///
    pub fn sip_string(text: &str) -> String {
        text.replace("|", "")
    }

    pub fn sip_date_now() -> String {
        Local::now().format(SIP_DATE_FORMAT).to_string()
    }

    /// Transltate an iso8601-ish to SIP format
    ///
    /// ```
    /// use sip2::util::Util;
    ///
    /// let date_op = Util::sip_date("1996-12-19T16:39:57-08:00");
    /// assert_eq!(date_op.is_ok(), true);
    ///
    /// let result = date_op.unwrap();
    /// assert_eq!(result, "19961219    163957");
    ///
    /// let date_op2 = Util::sip_date("YARP!");
    /// assert_eq!(date_op2.is_err(), true);
    /// ```
    pub fn sip_date(iso_date: &str) -> Result<String, error::Error> {
        match DateTime::parse_from_rfc3339(iso_date) {
            Ok(dt) => Ok(dt.format(SIP_DATE_FORMAT).to_string()),
            Err(s) => {
                println!("{}", s);
                Err(error::Error::DateFormatError)
            }
        }
    }
}


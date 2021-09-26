use std::collections::HashMap;
use chrono::{DateTime, Local};
use super::error::Error;
use std::fmt;

pub const LINE_TERMINATOR: char = '\r';
pub const SIP_DATE_FORMAT: &str = "%Y%m%d    %H%M%S";
pub const STRING_COLUMN_PAD: u8 = 32; // for printing/debugging

pub struct FixedField {
    pub label: &'static str,
    pub length: u8
}

impl fmt::Display for FixedField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.length)
    }
}

pub struct Field {
    pub label: &'static str,
    pub code: &'static str,
}

pub struct Message {
    pub code: &'static str,
    pub label: &'static str,
    pub fixed_fields: &'static [&'static FixedField],
    pub required_fields: &'static [&'static Field],
    pub optional_fields: &'static [&'static Field],
}

pub struct Util {
    field_map: HashMap<&'static str, &'static Field>,
    message_map: HashMap<&'static str, &'static Message>,
}

impl Util {

    pub fn new() -> Self {
        let mut u = Util {
            field_map: HashMap::new(),
            message_map: HashMap::new(),
        };
        u.set_field_map();
        u.set_message_map();
        u
    }

    fn set_field_map(&mut self) {
        self.field_map.insert(F_LOGIN_UID.code, &F_LOGIN_UID);
    }

    fn set_message_map(&mut self) {
        self.message_map.insert(M_LOGIN.code, &M_LOGIN);
        self.message_map.insert(M_SC_STATUS.code, &M_SC_STATUS);
    }

    pub fn get_field_by_code(&mut self, code: &str) -> Option<&Field> {
        // HashMap values are static refs.
        // HashMap.get() returns Option<&&Field>.
        // Unpack here with * cuz returning &&Field is weird
        match self.field_map.get(code) {
            Some(f) => Some(*f),
            None => None
        }
    }

    pub fn get_message_by_code(&self, code: &str) -> Option<&Message> {
        match self.message_map.get(code) {
            Some(m) => Some(*m),
            None => None
        }
    }

    pub fn escape_sip_text(text: &str) -> String {
        text.replace("|", "")
    }

    pub fn sip_date_now() -> String {
        Local::now().format(SIP_DATE_FORMAT).to_string()
    }

    pub fn sip_date(iso_date: &str) -> Result<String, Error> {
        match DateTime::parse_from_rfc3339(iso_date) {
            Ok(dt) => Ok(dt.format(SIP_DATE_FORMAT).to_string()),
            Err(_) => Err(Error::DateFormatError),
        }
    }
}

pub const FF_DATE:                  FixedField = FixedField { length: 18, label: "transaction date" };
pub const FF_OK:                    FixedField = FixedField { length: 1,  label: "ok" };
pub const FF_UID_ALGO:              FixedField = FixedField { length: 1,  label: "uid algorithm" };
pub const FF_PWD_ALGO:              FixedField = FixedField { length: 1,  label: "pwd algorithm" };
pub const FF_FEE_TYPE:              FixedField = FixedField { length: 2,  label: "fee type" };
pub const FF_PAYMENT_TYPE:          FixedField = FixedField { length: 2,  label: "payment type" };
pub const FF_CURRENCY:              FixedField = FixedField { length: 3,  label: "currency type" };
pub const FF_PAYMENT_ACCEPTED:      FixedField = FixedField { length: 1,  label: "payment accepted" };
pub const FF_CIRCULATION_STATUS:    FixedField = FixedField { length: 2,  label: "circulation status" };
pub const FF_SECURITY_MARKER:       FixedField = FixedField { length: 2,  label: "security marker" };
pub const FF_LANGUAGE:              FixedField = FixedField { length: 3,  label: "language" };
pub const FF_PATRON_STATUS:         FixedField = FixedField { length: 14, label: "patron status" };
pub const FF_SUMMARY:               FixedField = FixedField { length: 10, label: "summary" };
pub const FF_HOLD_ITEMS_COUNT:      FixedField = FixedField { length: 4,  label: "hold items count" };
pub const FF_OD_ITEMS_COUNT:        FixedField = FixedField { length: 4,  label: "overdue items count" };
pub const FF_CH_ITEMS_COUNT:        FixedField = FixedField { length: 4,  label: "charged items count" };
pub const FF_FINE_ITEMS_COUNT:      FixedField = FixedField { length: 4,  label: "fine items count" };
pub const FF_RECALL_ITEMS_COUNT:    FixedField = FixedField { length: 4,  label: "recall items count" };
pub const FF_UNAVAIL_ITEMS_COUNT:   FixedField = FixedField { length: 4,  label: "unavail holds count" };
pub const FF_RENEWAL_POLICY:        FixedField = FixedField { length: 1,  label: "sc renewal policy" };
pub const FF_NO_BLOCK:              FixedField = FixedField { length: 1,  label: "no block" };
pub const FF_NB_DUE_DATE:           FixedField = FixedField { length: 18, label: "nb due date" };
pub const FF_STATUS_CODE:           FixedField = FixedField { length: 1,  label: "status code" };
pub const FF_MAX_PRINT_WIDTH:       FixedField = FixedField { length: 3,  label: "max print width" };
pub const FF_PROTOCOL_VERSION:      FixedField = FixedField { length: 4,  label: "protocol version" };

/*
pub const FF_: FixedField = FixedField { length: 1,  label: "renewal ok" };
pub const FF_: FixedField = FixedField { length: 1,  label: "magnetic media" };
pub const FF_: FixedField = FixedField { length: 1,  label: "desensitize" };
pub const FF_: FixedField = FixedField { length: 1,  label: "resensitize" };
pub const FF_: FixedField = FixedField { length: 18, label: "return date" };
pub const FF_: FixedField = FixedField { length: 1,  label: "alert" };
pub const FF_: FixedField = FixedField { length: 1,  label: "on-line status" };
pub const FF_: FixedField = FixedField { length: 1,  label: "checkin ok" };
pub const FF_: FixedField = FixedField { length: 1,  label: "checkout ok" };
pub const FF_: FixedField = FixedField { length: 1,  label: "acs renewal policy" };
pub const FF_: FixedField = FixedField { length: 1,  label: "status update ok" };
pub const FF_: FixedField = FixedField { length: 1,  label: "offline ok" };
pub const FF_: FixedField = FixedField { length: 3,  label: "timeout period" };
pub const FF_: FixedField = FixedField { length: 3,  label: "retries allowed" };
pub const FF_: FixedField = FixedField { length: 18, label: "date/time sync" };
pub const FF_: FixedField = FixedField { length: 1,  label: "third party allowed" };
pub const FF_: FixedField = FixedField { length: 4,  label: "renewed count" };
pub const FF_: FixedField = FixedField { length: 4,  label: "unrenewed count" };
pub const FF_: FixedField = FixedField { length: 1,  label: "hold mode" };
pub const FF_: FixedField = FixedField { length: 1,  label: "hold available" };
*/

pub const F_LOGIN_UID:          Field = Field { code: "CN", label: "login user id" };
pub const F_LOGIN_PWD:          Field = Field { code: "CO", label: "login password" };
pub const F_PATRON_ID:          Field = Field { code: "AA", label: "patron identifier" };
pub const F_PATRON_IDENT:       Field = Field { code: "AA", label: "patron identifier" };
pub const F_ITEM_IDENT:         Field = Field { code: "AB", label: "item identifier" };
pub const F_TERMINAL_PWD:       Field = Field { code: "AC", label: "terminal password" };
pub const F_PATRON_PWD:         Field = Field { code: "AD", label: "patron password" };
pub const F_PERSONAL_NAME:      Field = Field { code: "AE", label: "personal name" };
pub const F_SCREEN_MSG:         Field = Field { code: "AF", label: "screen message" };
pub const F_PRINT_LINE:         Field = Field { code: "AG", label: "print line" };
pub const F_DUE_DATE:           Field = Field { code: "AH", label: "due date" };
pub const F_TITLE_IDENT:        Field = Field { code: "AJ", label: "title identifier" };
pub const F_BLOCKED_CARD_MSG:   Field = Field { code: "AL", label: "blocked card msg" };

/*
pub const F_: Field = Field { code: "AM", label: "library name" };
pub const F_: Field = Field { code: "AN", label: "terminal location" };
pub const F_: Field = Field { code: "AO", label: "institution id" };
pub const F_: Field = Field { code: "AP", label: "current location" };
pub const F_: Field = Field { code: "AQ", label: "permanent location" };
pub const F_: Field = Field { code: "AS", label: "hold items" };
pub const F_: Field = Field { code: "AT", label: "overdue items" };
pub const F_: Field = Field { code: "AU", label: "charged items" };
pub const F_: Field = Field { code: "AV", label: "fine items" };
pub const F_: Field = Field { code: "AY", label: "sequence number" };
pub const F_: Field = Field { code: "AZ", label: "checksum" };
pub const F_: Field = Field { code: "BD", label: "home address" };
pub const F_: Field = Field { code: "BE", label: "e-mail address" };
pub const F_: Field = Field { code: "BF", label: "home phone number" };
pub const F_: Field = Field { code: "BG", label: "owner" };
pub const F_: Field = Field { code: "BH", label: "currency type" };
pub const F_: Field = Field { code: "BI", label: "cancel" };
pub const F_: Field = Field { code: "BK", label: "transaction id" };
pub const F_: Field = Field { code: "BL", label: "valid patron" };
pub const F_: Field = Field { code: "BM", label: "renewed items" };
pub const F_: Field = Field { code: "BN", label: "unrenewed items" };
pub const F_: Field = Field { code: "BO", label: "fee acknowledged" };
pub const F_: Field = Field { code: "BP", label: "start item" };
pub const F_: Field = Field { code: "BQ", label: "end item" };
pub const F_: Field = Field { code: "BR", label: "queue position" };
pub const F_: Field = Field { code: "BS", label: "pickup location" };
pub const F_: Field = Field { code: "BT", label: "fee type" };
pub const F_: Field = Field { code: "BU", label: "recall items" };
pub const F_: Field = Field { code: "BV", label: "fee amount" };
pub const F_: Field = Field { code: "BW", label: "expiration date" };
pub const F_: Field = Field { code: "BX", label: "supported messages" };
pub const F_: Field = Field { code: "BY", label: "hold type" };
pub const F_: Field = Field { code: "BZ", label: "hold items limit" };
pub const F_: Field = Field { code: "CA", label: "overdue items limit" };
pub const F_: Field = Field { code: "CB", label: "charged items limit" };
pub const F_: Field = Field { code: "CC", label: "fee limit" };
pub const F_: Field = Field { code: "CD", label: "unavailable hold items" };
pub const F_: Field = Field { code: "CF", label: "hold queue length" };
pub const F_: Field = Field { code: "CG", label: "fee identifier" };
pub const F_: Field = Field { code: "CH", label: "item properties" };
pub const F_: Field = Field { code: "CI", label: "security inhibit" };
pub const F_: Field = Field { code: "CJ", label: "recall date" };
pub const F_: Field = Field { code: "CK", label: "media type" };
pub const F_: Field = Field { code: "CL", label: "sort bin" };
pub const F_: Field = Field { code: "CM", label: "hold pickup date" };
pub const F_: Field = Field { code: "CN", label: "login user id" };
pub const F_: Field = Field { code: "CO", label: "login password" };
pub const F_: Field = Field { code: "CP", label: "location code" };
pub const F_: Field = Field { code: "CQ", label: "valid patron password" };
pub const F_: Field = Field { code: "PI", label: "patron internet profile" };
pub const F_: Field = Field { code: "CS", label: "call number" };
pub const F_: Field = Field { code: "CR", label: "collection code" };
pub const F_: Field = Field { code: "CV", label: "alert type" };
pub const F_: Field = Field { code: "CY", label: "hold patron id" };
pub const F_: Field = Field { code: "DA", label: "hold patron name" };
pub const F_: Field = Field { code: "CT", label: "destination location" };

//  Envisionware Terminal Extensions
pub const F_: Field = Field { code: "PA", label: "patron expire date" };
pub const F_: Field = Field { code: "PB", label: "patron birth date" };
pub const F_: Field = Field { code: "PC", label: "patron class" };
pub const F_: Field = Field { code: "OR", label: "register login" };
pub const F_: Field = Field { code: "RN", label: "check number" };
*/

const EMPTY: [&'static Field; 0] = [];

pub const M_SC_STATUS: Message = Message {
    code: "99",
    label: "SC Status",
    fixed_fields: &[
        &FF_STATUS_CODE,
        &FF_MAX_PRINT_WIDTH,
        &FF_PROTOCOL_VERSION
    ],
    required_fields: &EMPTY,
    optional_fields: &EMPTY,
};

pub const M_LOGIN: Message = Message {
    code: "93",
    label: "Login Request",
    fixed_fields: &[&FF_UID_ALGO, &FF_PWD_ALGO],
    required_fields: &[&F_LOGIN_UID, &F_LOGIN_PWD],
    optional_fields: &EMPTY,
};




use std::collections::HashMap;
use super::error::Error;
use std::fmt;

pub const SIP_PROTOCOL_VERSION: &str = "2.00";
pub const LINE_TERMINATOR: char = '\r';
pub const SIP_DATE_FORMAT: &str = "%Y%m%d    %H%M%S";

pub struct FixedField {
    pub label: &'static str,
    pub length: usize,
}

impl fmt::Display for FixedField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.length)
    }
}

pub struct FixedField2 {
    pub label: String,
    pub length: usize,
}

impl fmt::Display for FixedField2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.length)
    }
}

pub struct Field2 {
    pub label: String,
    pub code: String,
}

impl fmt::Display for Field2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.label)
    }
}

pub struct Field {
    pub label: &'static str,
    pub code: &'static str,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.label)
    }
}

impl Field {

    /// Get a Field from its 2-character code.
    ///
    /// ```
    /// use sip2::spec;
    /// let f = &spec::F_LOGIN_UID;
    /// let f2 = spec::Field::from_code(f.code).unwrap();
    /// assert_eq!(f2.code, f.code);
    /// ```
    pub fn from_code(code: &str) -> Option<&'static Field> {
        match code {
            f if f == F_LOGIN_UID.code              => Some(&F_LOGIN_UID),
            f if f == F_LOGIN_PWD.code              => Some(&F_LOGIN_PWD),
            f if f == F_PATRON_ID.code              => Some(&F_PATRON_ID),
            f if f == F_PATRON_IDENT.code           => Some(&F_PATRON_IDENT),
            f if f == F_ITEM_IDENT.code             => Some(&F_ITEM_IDENT),
            f if f == F_TERMINAL_PWD.code           => Some(&F_TERMINAL_PWD),
            f if f == F_PATRON_PWD.code             => Some(&F_PATRON_PWD),
            f if f == F_PERSONAL_NAME.code          => Some(&F_PERSONAL_NAME),
            f if f == F_SCREEN_MSG.code             => Some(&F_SCREEN_MSG),
            f if f == F_PRINT_LINE.code             => Some(&F_PRINT_LINE),
            f if f == F_DUE_DATE.code               => Some(&F_DUE_DATE),
            f if f == F_TITLE_IDENT.code            => Some(&F_TITLE_IDENT),
            f if f == F_BLOCKED_CARD_MSG.code       => Some(&F_BLOCKED_CARD_MSG),
            f if f == F_LIBRARY_NAME.code           => Some(&F_LIBRARY_NAME),
            f if f == F_TERMINAL_LOCATION.code      => Some(&F_TERMINAL_LOCATION),
            f if f == F_INSTITUTION_ID.code         => Some(&F_INSTITUTION_ID),
            f if f == F_CURRENT_LOCATION.code       => Some(&F_CURRENT_LOCATION),
            f if f == F_PERMANENT_LOCATION.code     => Some(&F_PERMANENT_LOCATION),
            f if f == F_HOLD_ITEMS.code             => Some(&F_HOLD_ITEMS),
            f if f == F_OVERDUE_ITEMS.code          => Some(&F_OVERDUE_ITEMS),
            f if f == F_CHARGED_ITEMS.code          => Some(&F_CHARGED_ITEMS),
            f if f == F_FINE_ITEMS.code             => Some(&F_FINE_ITEMS),
            f if f == F_SEQUENCE_NUMBER.code        => Some(&F_SEQUENCE_NUMBER),
            f if f == F_CHECKSUM.code               => Some(&F_CHECKSUM),
            f if f == F_HOME_ADDRESS.code           => Some(&F_HOME_ADDRESS),
            f if f == F_EMAIL_ADDRESS.code          => Some(&F_EMAIL_ADDRESS),
            f if f == F_HOME_PHONE.code             => Some(&F_HOME_PHONE),
            f if f == F_OWNER.code                  => Some(&F_OWNER),
            f if f == F_CURRENCY.code               => Some(&F_CURRENCY),
            f if f == F_CANCEL.code                 => Some(&F_CANCEL),
            f if f == F_TRANSACTION_ID.code         => Some(&F_TRANSACTION_ID),
            f if f == F_VALID_PATRON.code           => Some(&F_VALID_PATRON),
            f if f == F_RENEWED_ITEMS.code          => Some(&F_RENEWED_ITEMS),
            f if f == F_UNRENEWED_ITEMS.code        => Some(&F_UNRENEWED_ITEMS),
            f if f == F_FEE_ACKNOWLEGED.code        => Some(&F_FEE_ACKNOWLEGED),
            f if f == F_START_ITEM.code             => Some(&F_START_ITEM),
            f if f == F_END_ITEM.code               => Some(&F_END_ITEM),
            f if f == F_QUEUE_POSITION.code         => Some(&F_QUEUE_POSITION),
            f if f == F_PICKUP_LOCATION.code        => Some(&F_PICKUP_LOCATION),
            f if f == F_RECALL_ITEMS.code           => Some(&F_RECALL_ITEMS),
            f if f == F_FEE_TYPE.code               => Some(&F_FEE_TYPE),
            f if f == F_FEE_LIMIT.code              => Some(&F_FEE_LIMIT),
            f if f == F_FEE_AMOUNT.code             => Some(&F_FEE_AMOUNT),
            f if f == F_EXPIRE_DATE.code            => Some(&F_EXPIRE_DATE),
            f if f == F_SUPPORTED_MESSAGES.code     => Some(&F_SUPPORTED_MESSAGES),
            f if f == F_HOLD_TYPE.code              => Some(&F_HOLD_TYPE),
            f if f == F_HOLD_ITEMS_LIMIT.code       => Some(&F_HOLD_ITEMS_LIMIT),
            f if f == F_OVERDUE_ITEMS_LIST.code     => Some(&F_OVERDUE_ITEMS_LIST),
            f if f == F_CHARGED_ITEMS_LIMIT.code    => Some(&F_CHARGED_ITEMS_LIMIT),
            f if f == F_UNAVAIL_HOLD_ITEMS.code     => Some(&F_UNAVAIL_HOLD_ITEMS),
            f if f == F_HOLD_QUEUE_LENGTH.code      => Some(&F_HOLD_QUEUE_LENGTH),
            f if f == F_FEE_IDENTIFIER.code         => Some(&F_FEE_IDENTIFIER),
            f if f == F_ITEM_PROPERTIES.code        => Some(&F_ITEM_PROPERTIES),
            f if f == F_SECURITY_INHIBIT.code       => Some(&F_SECURITY_INHIBIT),
            f if f == F_RECALL_DATE.code            => Some(&F_RECALL_DATE),
            f if f == F_MEDIA_TYPE.code             => Some(&F_MEDIA_TYPE),
            f if f == F_SORT_BIN.code               => Some(&F_SORT_BIN),
            f if f == F_HOLD_PICKUP_DATE.code       => Some(&F_HOLD_PICKUP_DATE),
            f if f == F_LOGIN_USER_ID.code          => Some(&F_LOGIN_USER_ID),
            f if f == F_LOCATION_CODE.code          => Some(&F_LOCATION_CODE),
            f if f == F_VALID_PATRON_PWD.code       => Some(&F_VALID_PATRON_PWD),
            f if f == F_INET_PROFILE.code           => Some(&F_INET_PROFILE),
            f if f == F_CALL_NUMBER.code            => Some(&F_CALL_NUMBER),
            f if f == F_COLLECTION_CODE.code        => Some(&F_COLLECTION_CODE),
            f if f == F_ALERT_TYPE.code             => Some(&F_ALERT_TYPE),
            f if f == F_HOLD_PATRON_ID.code         => Some(&F_HOLD_PATRON_ID),
            f if f == F_HOLD_PATRON_NAME.code       => Some(&F_HOLD_PATRON_NAME),
            f if f == F_DEST_LOCATION.code          => Some(&F_DEST_LOCATION),
            f if f == F_PATRON_EXPIRE_DATE.code     => Some(&F_PATRON_EXPIRE_DATE),
            f if f == F_PATRON_DOB.code             => Some(&F_PATRON_DOB),
            f if f == F_PATRON_CLASS.code           => Some(&F_PATRON_CLASS),
            f if f == F_REGISTER_LOGIN.code         => Some(&F_REGISTER_LOGIN),
            f if f == F_CHECK_NUMBER.code           => Some(&F_CHECK_NUMBER),
            _ => None
        }
    }
}

pub struct Message2 {
    pub code: String,
    pub label: String,
    pub fixed_fields: Vec<&FixedField2>,
}

pub struct Message {
    pub code: &'static str,
    pub label: &'static str,
    pub fixed_fields: &'static [&'static FixedField],
}

impl Message {

    /// Maps a message code to a message spec.
    ///
    /// ```
    /// use sip2::spec;
    /// let msg = &spec::M_LOGIN;
    /// let msg2 = spec::Message::from_code(&spec::M_LOGIN.code).unwrap();
    /// assert_eq!(msg2.code, msg.code);
    /// ```
    pub fn from_code(code: &str) -> Option<&'static Message> {

        match code {
            m if m == M_SC_STATUS.code          => Some(&M_SC_STATUS),
            m if m == M_ACS_STATUS.code         => Some(&M_ACS_STATUS),
            m if m == M_LOGIN.code              => Some(&M_LOGIN),
            m if m == M_LOGIN_RESP.code         => Some(&M_LOGIN_RESP),
            m if m == M_ITEM_INFO.code          => Some(&M_ITEM_INFO),
            m if m == M_ITEM_INFO_RESP.code     => Some(&M_ITEM_INFO_RESP),
            m if m == M_PATRON_STATUS.code      => Some(&M_PATRON_STATUS),
            m if m == M_PATRON_STATUS_RESP.code => Some(&M_PATRON_STATUS_RESP),
            m if m == M_PATRON_INFO.code        => Some(&M_PATRON_INFO),
            m if m == M_PATRON_INFO_RESP.code   => Some(&M_PATRON_INFO_RESP),
            m if m == M_CHECKOUT.code           => Some(&M_CHECKOUT),
            m if m == M_CHECKOUT_RESP.code      => Some(&M_CHECKOUT_RESP),
            m if m == M_RENEW.code              => Some(&M_RENEW),
            m if m == M_RENEW_RESP.code         => Some(&M_RENEW_RESP),
            m if m == M_RENEW_ALL.code          => Some(&M_RENEW_ALL),
            m if m == M_RENEW_ALL_RESP.code     => Some(&M_RENEW_ALL_RESP),
            m if m == M_CHECKIN.code            => Some(&M_CHECKIN),
            m if m == M_CHECKIN_RESP.code       => Some(&M_CHECKIN_RESP),
            m if m == M_HOLD.code               => Some(&M_HOLD),
            m if m == M_HOLD_RESP.code          => Some(&M_HOLD_RESP),
            m if m == M_FEE_PAID.code           => Some(&M_FEE_PAID),
            m if m == M_FEE_PAID_RESP.code      => Some(&M_FEE_PAID_RESP),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------
// Fixed Fields
// -------------------------------------------------------------------------

type FF = FixedField; // local shorthand

pub const FF_DATE                : FF = FF { length: 18, label: "transaction date" };
pub const FF_OK                  : FF = FF { length: 1,  label: "ok" };
pub const FF_UID_ALGO            : FF = FF { length: 1,  label: "uid algorithm" };
pub const FF_PWD_ALGO            : FF = FF { length: 1,  label: "pwd algorithm" };
pub const FF_FEE_TYPE            : FF = FF { length: 2,  label: "fee type" };
pub const FF_PAYMENT_TYPE        : FF = FF { length: 2,  label: "payment type" };
pub const FF_CURRENCY            : FF = FF { length: 3,  label: "currency type" };
pub const FF_PAYMENT_ACCEPTED    : FF = FF { length: 1,  label: "payment accepted" };
pub const FF_CIRCULATION_STATUS  : FF = FF { length: 2,  label: "circulation status" };
pub const FF_SECURITY_MARKER     : FF = FF { length: 2,  label: "security marker" };
pub const FF_LANGUAGE            : FF = FF { length: 3,  label: "language" };
pub const FF_PATRON_STATUS       : FF = FF { length: 14, label: "patron status" };
pub const FF_SUMMARY             : FF = FF { length: 10, label: "summary" };
pub const FF_HOLD_ITEMS_COUNT    : FF = FF { length: 4,  label: "hold items count" };
pub const FF_OD_ITEMS_COUNT      : FF = FF { length: 4,  label: "overdue items count" };
pub const FF_CH_ITEMS_COUNT      : FF = FF { length: 4,  label: "charged items count" };
pub const FF_FINE_ITEMS_COUNT    : FF = FF { length: 4,  label: "fine items count" };
pub const FF_RECALL_ITEMS_COUNT  : FF = FF { length: 4,  label: "recall items count" };
pub const FF_UNAVAIL_HOLDS_COUNT : FF = FF { length: 4,  label: "unavail holds count" };
pub const FF_SC_RENEWAL_POLICY   : FF = FF { length: 1,  label: "sc renewal policy" };
pub const FF_NO_BLOCK            : FF = FF { length: 1,  label: "no block" };
pub const FF_NB_DUE_DATE         : FF = FF { length: 18, label: "nb due date" };
pub const FF_STATUS_CODE         : FF = FF { length: 1,  label: "status code" };
pub const FF_MAX_PRINT_WIDTH     : FF = FF { length: 3,  label: "max print width" };
pub const FF_PROTOCOL_VERSION    : FF = FF { length: 4,  label: "protocol version" };
pub const FF_RENEW_OK            : FF = FF { length: 1,  label: "renewal ok" };
pub const FF_MAGNETIC_MEDIA      : FF = FF { length: 1,  label: "magnetic media" };
pub const FF_DESENSITIZE         : FF = FF { length: 1,  label: "desensitize" };
pub const FF_RESENSITIZE         : FF = FF { length: 1,  label: "resensitize" };
pub const FF_RETURN_DATE         : FF = FF { length: 18, label: "return date" };
pub const FF_ALERT               : FF = FF { length: 1,  label: "alert" };
pub const FF_ONLINE_STATUS       : FF = FF { length: 1,  label: "on-line status" };
pub const FF_CHECKIN_OK          : FF = FF { length: 1,  label: "checkin ok" };
pub const FF_CHECKOUT_OK         : FF = FF { length: 1,  label: "checkout ok" };
pub const FF_ACS_RENEWAL_POLICY  : FF = FF { length: 1,  label: "acs renewal policy" };
pub const FF_STATUS_UPDATE_OK    : FF = FF { length: 1,  label: "status update ok" };
pub const FF_OFFLINE_OK          : FF = FF { length: 1,  label: "offline ok" };
pub const FF_TIMEOUT_PERIOD      : FF = FF { length: 3,  label: "timeout period" };
pub const FF_RETRIES_ALLOWED     : FF = FF { length: 3,  label: "retries allowed" };
pub const FF_DATETIME_SYNC       : FF = FF { length: 18, label: "date/time sync" };
pub const FF_THIRD_PARTY_ALLOWED : FF = FF { length: 1,  label: "third party allowed" };
pub const FF_RENEWED_COUNT       : FF = FF { length: 4,  label: "renewed count" };
pub const FF_UNRENEWED_COUNT     : FF = FF { length: 4,  label: "unrenewed count" };
pub const FF_HOLD_MODE           : FF = FF { length: 1,  label: "hold mode" };
pub const FF_HOLD_AVAILABLE      : FF = FF { length: 1,  label: "hold available" };

// -------------------------------------------------------------------------
// Fields
// -------------------------------------------------------------------------

type F = Field; // local shorthand

pub const F_LOGIN_UID               : F = F { code: "CN", label: "login user id" };
pub const F_LOGIN_PWD               : F = F { code: "CO", label: "login password" };
pub const F_PATRON_ID               : F = F { code: "AA", label: "patron identifier" };
pub const F_PATRON_IDENT            : F = F { code: "AA", label: "patron identifier" };
pub const F_ITEM_IDENT              : F = F { code: "AB", label: "item identifier" };
pub const F_TERMINAL_PWD            : F = F { code: "AC", label: "terminal password" };
pub const F_PATRON_PWD              : F = F { code: "AD", label: "patron password" };
pub const F_PERSONAL_NAME           : F = F { code: "AE", label: "personal name" };
pub const F_SCREEN_MSG              : F = F { code: "AF", label: "screen message" };
pub const F_PRINT_LINE              : F = F { code: "AG", label: "print line" };
pub const F_DUE_DATE                : F = F { code: "AH", label: "due date" };
pub const F_TITLE_IDENT             : F = F { code: "AJ", label: "title identifier" };
pub const F_BLOCKED_CARD_MSG        : F = F { code: "AL", label: "blocked card msg" };
pub const F_LIBRARY_NAME            : F = F { code: "AM", label: "library name" };
pub const F_TERMINAL_LOCATION       : F = F { code: "AN", label: "terminal location" };
pub const F_INSTITUTION_ID          : F = F { code: "AO", label: "institution id" };
pub const F_CURRENT_LOCATION        : F = F { code: "AP", label: "current location" };
pub const F_PERMANENT_LOCATION      : F = F { code: "AQ", label: "permanent location" };
pub const F_HOLD_ITEMS              : F = F { code: "AS", label: "hold items" };
pub const F_OVERDUE_ITEMS           : F = F { code: "AT", label: "overdue items" };
pub const F_CHARGED_ITEMS           : F = F { code: "AU", label: "charged items" };
pub const F_FINE_ITEMS              : F = F { code: "AV", label: "fine items" };
pub const F_SEQUENCE_NUMBER         : F = F { code: "AY", label: "sequence number" };
pub const F_CHECKSUM                : F = F { code: "AZ", label: "checksum" };
pub const F_HOME_ADDRESS            : F = F { code: "BD", label: "home address" };
pub const F_EMAIL_ADDRESS           : F = F { code: "BE", label: "e-mail address" };
pub const F_HOME_PHONE              : F = F { code: "BF", label: "home phone number" };
pub const F_OWNER                   : F = F { code: "BG", label: "owner" };
pub const F_CURRENCY                : F = F { code: "BH", label: "currency type" };
pub const F_CANCEL                  : F = F { code: "BI", label: "cancel" };
pub const F_TRANSACTION_ID          : F = F { code: "BK", label: "transaction id" };
pub const F_VALID_PATRON            : F = F { code: "BL", label: "valid patron" };
pub const F_RENEWED_ITEMS           : F = F { code: "BM", label: "renewed items" };
pub const F_UNRENEWED_ITEMS         : F = F { code: "BN", label: "unrenewed items" };
pub const F_FEE_ACKNOWLEGED         : F = F { code: "BO", label: "fee acknowledged" };
pub const F_START_ITEM              : F = F { code: "BP", label: "start item" };
pub const F_END_ITEM                : F = F { code: "BQ", label: "end item" };
pub const F_QUEUE_POSITION          : F = F { code: "BR", label: "queue position" };
pub const F_PICKUP_LOCATION         : F = F { code: "BS", label: "pickup location" };
pub const F_RECALL_ITEMS            : F = F { code: "BU", label: "recall items" };
pub const F_FEE_TYPE                : F = F { code: "BT", label: "fee type" };
pub const F_FEE_LIMIT               : F = F { code: "CC", label: "fee limit" };
pub const F_FEE_AMOUNT              : F = F { code: "BV", label: "fee amount" };
pub const F_EXPIRE_DATE             : F = F { code: "BW", label: "expiration date" };
pub const F_SUPPORTED_MESSAGES      : F = F { code: "BX", label: "supported messages" };
pub const F_HOLD_TYPE               : F = F { code: "BY", label: "hold type" };
pub const F_HOLD_ITEMS_LIMIT        : F = F { code: "BZ", label: "hold items limit" };
pub const F_OVERDUE_ITEMS_LIST      : F = F { code: "CA", label: "overdue items limit" };
pub const F_CHARGED_ITEMS_LIMIT     : F = F { code: "CB", label: "charged items limit" };
pub const F_UNAVAIL_HOLD_ITEMS      : F = F { code: "CD", label: "unavailable hold items" };
pub const F_HOLD_QUEUE_LENGTH       : F = F { code: "CF", label: "hold queue length" };
pub const F_FEE_IDENTIFIER          : F = F { code: "CG", label: "fee identifier" };
pub const F_ITEM_PROPERTIES         : F = F { code: "CH", label: "item properties" };
pub const F_SECURITY_INHIBIT        : F = F { code: "CI", label: "security inhibit" };
pub const F_RECALL_DATE             : F = F { code: "CJ", label: "recall date" };
pub const F_MEDIA_TYPE              : F = F { code: "CK", label: "media type" };
pub const F_SORT_BIN                : F = F { code: "CL", label: "sort bin" };
pub const F_HOLD_PICKUP_DATE        : F = F { code: "CM", label: "hold pickup date" };
pub const F_LOGIN_USER_ID           : F = F { code: "CN", label: "login user id" };
pub const F_LOCATION_CODE           : F = F { code: "CP", label: "location code" };
pub const F_VALID_PATRON_PWD        : F = F { code: "CQ", label: "valid patron password" };
pub const F_INET_PROFILE            : F = F { code: "PI", label: "patron internet profile" };
pub const F_CALL_NUMBER             : F = F { code: "CS", label: "call number" };
pub const F_COLLECTION_CODE         : F = F { code: "CR", label: "collection code" };
pub const F_ALERT_TYPE              : F = F { code: "CV", label: "alert type" };
pub const F_HOLD_PATRON_ID          : F = F { code: "CY", label: "hold patron id" };
pub const F_HOLD_PATRON_NAME        : F = F { code: "DA", label: "hold patron name" };
pub const F_DEST_LOCATION           : F = F { code: "CT", label: "destination location" };

//  Envisionware Terminal Extensions
pub const F_PATRON_EXPIRE_DATE      : F = F { code: "PA", label: "patron expire date" };
pub const F_PATRON_DOB              : F = F { code: "PB", label: "patron birth date" };
pub const F_PATRON_CLASS            : F = F { code: "PC", label: "patron class" };
pub const F_REGISTER_LOGIN          : F = F { code: "OR", label: "register login" };
pub const F_CHECK_NUMBER            : F = F { code: "RN", label: "check number" };

// NOTE: when adding new fields, be sure to also add the new
// to Field::from_code()

// -------------------------------------------------------------------------
// Messages
// -------------------------------------------------------------------------

pub const EMPTY: &[&FixedField; 0] = &[];

pub const M_SC_STATUS: Message = Message {
    code: "99",
    label: "SC Status",
    fixed_fields: &[
        &FF_STATUS_CODE,
        &FF_MAX_PRINT_WIDTH,
        &FF_PROTOCOL_VERSION
    ],
};

pub const M_ACS_STATUS: Message = Message {
    code: "98",
    label: "ACS Status",
    fixed_fields: &[
        &FF_ONLINE_STATUS,
        &FF_CHECKIN_OK,
        &FF_CHECKOUT_OK,
        &FF_ACS_RENEWAL_POLICY,
        &FF_STATUS_UPDATE_OK,
        &FF_OFFLINE_OK,
        &FF_TIMEOUT_PERIOD,
        &FF_RETRIES_ALLOWED,
        &FF_DATETIME_SYNC,
        &FF_PROTOCOL_VERSION,
    ],
};

pub const M_LOGIN: Message = Message {
    code: "93",
    label: "Login Request",
    fixed_fields: &[&FF_UID_ALGO, &FF_PWD_ALGO],
};

pub const M_LOGIN_RESP: Message = Message {
    code: "94",
    label: "Login Response",
    fixed_fields: &[&FF_OK],
};

pub const M_ITEM_INFO: Message = Message {
    code: "17",
    label: "Item Information Request",
    fixed_fields: &[&FF_DATE],
};

pub const M_ITEM_INFO_RESP: Message = Message {
    code: "18",
    label: "Item Information Response",
    fixed_fields: &[
        &FF_CIRCULATION_STATUS,
        &FF_SECURITY_MARKER,
        &FF_FEE_TYPE,
        &FF_DATE,
    ],
};

pub const M_PATRON_STATUS: Message = Message {
    code: "23",
    label: "Patron Status Request",
    fixed_fields: &[
        &FF_LANGUAGE,
        &FF_DATE,
    ],
};

pub const M_PATRON_STATUS_RESP: Message = Message {
    code: "24",
    label: "Patron Status Response",
    fixed_fields: &[
        &FF_PATRON_STATUS,
        &FF_LANGUAGE,
        &FF_DATE,
    ],
};

pub const M_PATRON_INFO: Message = Message {
    code: "63",
    label: "Patron Information",
    fixed_fields: &[
        &FF_LANGUAGE,
        &FF_DATE,
        &FF_SUMMARY,
    ],
};

pub const M_PATRON_INFO_RESP: Message = Message {
    code: "64",
    label: "Patron Information Response",
    fixed_fields: &[
        &FF_PATRON_STATUS,
        &FF_LANGUAGE,
        &FF_DATE,
        &FF_HOLD_ITEMS_COUNT,
        &FF_OD_ITEMS_COUNT,
        &FF_CH_ITEMS_COUNT,
        &FF_FINE_ITEMS_COUNT,
        &FF_RECALL_ITEMS_COUNT,
        &FF_UNAVAIL_HOLDS_COUNT,
    ],
};

pub const M_CHECKOUT: Message = Message {
    code: "11",
    label: "Checkout Request",
    fixed_fields: &[
        &FF_SC_RENEWAL_POLICY,
        &FF_NO_BLOCK,
        &FF_DATE,
        &FF_NB_DUE_DATE,
    ],
};

pub const M_CHECKOUT_RESP: Message = Message {
    code: "12",
    label: "Checkout Response",
    fixed_fields: &[
        &FF_OK,
        &FF_RENEW_OK,
        &FF_MAGNETIC_MEDIA,
        &FF_DESENSITIZE,
        &FF_DATE,
    ],
};

pub const M_RENEW: Message = Message {
    code: "29",
    label: "Renew Request",
    fixed_fields: &[
        &FF_THIRD_PARTY_ALLOWED,
        &FF_NO_BLOCK,
        &FF_DATE,
        &FF_NB_DUE_DATE,
    ],
};

pub const M_RENEW_RESP: Message = Message {
    code: "12",
    label: "Checkout Response",
    fixed_fields: &[
        &FF_OK,
        &FF_RENEW_OK,
        &FF_MAGNETIC_MEDIA,
        &FF_DESENSITIZE,
        &FF_DATE,
    ],
};

pub const M_RENEW_ALL: Message = Message {
    code: "65",
    label: "Renew All Request",
    fixed_fields: &[&FF_DATE],
};

pub const M_RENEW_ALL_RESP: Message = Message {
    code: "66",
    label: "Renew All Response",
    fixed_fields: &[
        &FF_OK,
        &FF_RENEWED_COUNT,
        &FF_UNRENEWED_COUNT,
        &FF_DATE,
    ],
};

pub const M_CHECKIN: Message = Message {
    code: "09",
    label: "Checkin Request",
    fixed_fields: &[
        &FF_NO_BLOCK,
        &FF_DATE,
        &FF_RETURN_DATE
    ]
};

pub const M_CHECKIN_RESP: Message = Message {
    code: "10",
    label: "Checkin Response",
    fixed_fields: &[
        &FF_OK,
        &FF_RESENSITIZE,
        &FF_MAGNETIC_MEDIA,
        &FF_ALERT,
        &FF_DATE
    ]
};

pub const M_HOLD: Message = Message {
    code: "15",
    label: "Hold Request",
    fixed_fields: &[
        &FF_HOLD_MODE,
        &FF_DATE,
    ]
};

pub const M_HOLD_RESP: Message = Message {
    code: "16",
    label: "Hold Response",
    fixed_fields: &[
        &FF_OK,
        &FF_HOLD_AVAILABLE,
        &FF_DATE,
    ]
};


pub const M_FEE_PAID: Message = Message {
    code: "37",
    label: "Fee Paid",
    fixed_fields: &[
        &FF_DATE,
        &FF_FEE_TYPE,
        &FF_PAYMENT_TYPE,
        &FF_CURRENCY,
    ]
};

pub const M_FEE_PAID_RESP: Message = Message {
    code: "38",
    label: "Fee Paid Response",
    fixed_fields: &[
        &FF_PAYMENT_ACCEPTED,
        &FF_DATE
    ]
};


// NOTE: when adding new message types, be sure to also add the new
// message to Message::from_code()

pub struct Spec {
    fixed_fields: HashMap<String, FixedField2>,
    fields: HashMap<String, Field2>,
    messages: HashMap<String, Message2>
}

// Local shortcut for to_string() to make the pile-o-spec's a little
// bit shorter.
fn s(st: &str) -> String { st.to_string() }

impl Spec {

    pub fn new() -> Self {
        let mut spec = Spec {
            fixed_fields: HashMap::new(),
            fields: HashMap::new(),
            messages: HashMap::new(),
        };

        spec.set_fixed_fields();
        spec.set_fields();
        spec.set_messages();
        spec
    }

    fn set_fixed_fields(&mut self) {

        let ff = &mut self.fixed_fields;    // local shorthand
        type FF = FixedField2;              // local shorthand

        ff.insert(s("date")                , FF { length: 18, label: s("transaction date") });
        ff.insert(s("ok")                  , FF { length: 1,  label: s("ok") });
        ff.insert(s("uid_algo")            , FF { length: 1,  label: s("uid algorithm") });
        ff.insert(s("pwd_algo")            , FF { length: 1,  label: s("pwd algorithm") });
        ff.insert(s("fee_type")            , FF { length: 2,  label: s("fee type") });
        ff.insert(s("payment_type")        , FF { length: 2,  label: s("payment type") });
        ff.insert(s("currency")            , FF { length: 3,  label: s("currency type") });
        ff.insert(s("payment_accepted")    , FF { length: 1,  label: s("payment accepted") });
        ff.insert(s("circulation_status")  , FF { length: 2,  label: s("circulation status") });
        ff.insert(s("security_marker")     , FF { length: 2,  label: s("security marker") });
        ff.insert(s("language")            , FF { length: 3,  label: s("language") });
        ff.insert(s("patron_status")       , FF { length: 14, label: s("patron status") });
        ff.insert(s("summary")             , FF { length: 10, label: s("summary") });
        ff.insert(s("hold_items_count")    , FF { length: 4,  label: s("hold items count") });
        ff.insert(s("od_items_count")      , FF { length: 4,  label: s("overdue items count") });
        ff.insert(s("ch_items_count")      , FF { length: 4,  label: s("charged items count") });
        ff.insert(s("fine_items_count")    , FF { length: 4,  label: s("fine items count") });
        ff.insert(s("recall_items_count")  , FF { length: 4,  label: s("recall items count") });
        ff.insert(s("unavail_holds_count") , FF { length: 4,  label: s("unavail holds count") });
        ff.insert(s("sc_renewal_policy")   , FF { length: 1,  label: s("sc renewal policy") });
        ff.insert(s("no_block")            , FF { length: 1,  label: s("no block") });
        ff.insert(s("nb_due_date")         , FF { length: 18, label: s("nb due date") });
        ff.insert(s("status_code")         , FF { length: 1,  label: s("status code") });
        ff.insert(s("max_print_width")     , FF { length: 3,  label: s("max print width") });
        ff.insert(s("protocol_version")    , FF { length: 4,  label: s("protocol version") });
        ff.insert(s("renew_ok")            , FF { length: 1,  label: s("renewal ok") });
        ff.insert(s("magnetic_media")      , FF { length: 1,  label: s("magnetic media") });
        ff.insert(s("desensitize")         , FF { length: 1,  label: s("desensitize") });
        ff.insert(s("resensitize")         , FF { length: 1,  label: s("resensitize") });
        ff.insert(s("return_date")         , FF { length: 18, label: s("return date") });
        ff.insert(s("alert")               , FF { length: 1,  label: s("alert") });
        ff.insert(s("online_status")       , FF { length: 1,  label: s("on-line status") });
        ff.insert(s("checkin_ok")          , FF { length: 1,  label: s("checkin ok") });
        ff.insert(s("checkout_ok")         , FF { length: 1,  label: s("checkout ok") });
        ff.insert(s("acs_renewal_policy")  , FF { length: 1,  label: s("acs renewal policy") });
        ff.insert(s("status_update_ok")    , FF { length: 1,  label: s("status update ok") });
        ff.insert(s("offline_ok")          , FF { length: 1,  label: s("offline ok") });
        ff.insert(s("timeout_period")      , FF { length: 3,  label: s("timeout period") });
        ff.insert(s("retries_allowed")     , FF { length: 3,  label: s("retries allowed") });
        ff.insert(s("datetime_sync")       , FF { length: 18, label: s("date/time sync") });
        ff.insert(s("third_party_allowed") , FF { length: 1,  label: s("third party allowed") });
        ff.insert(s("renewed_count")       , FF { length: 4,  label: s("renewed count") });
        ff.insert(s("unrenewed_count")     , FF { length: 4,  label: s("unrenewed count") });
        ff.insert(s("hold_mode")           , FF { length: 1,  label: s("hold mode") });
        ff.insert(s("hold_available")      , FF { length: 1,  label: s("hold available") });
    }

    pub fn fixed_field_by_code(&self, code: &str) -> Option<&FixedField2> {
        self.fixed_fields.get(code)
    }

    fn set_fields(&mut self) {

        let f = &mut self.fields;   // local shorthand
        type F = Field2;            // local shorthand

        f.insert(s("login_uid")               , F { code: s("CN"), label: s("login user id") });
        f.insert(s("login_pwd")               , F { code: s("CO"), label: s("login password") });
        f.insert(s("patron_id")               , F { code: s("AA"), label: s("patron identifier") });
        f.insert(s("patron_ident")            , F { code: s("AA"), label: s("patron identifier") });
        f.insert(s("item_ident")              , F { code: s("AB"), label: s("item identifier") });
        f.insert(s("terminal_pwd")            , F { code: s("AC"), label: s("terminal password") });
        f.insert(s("patron_pwd")              , F { code: s("AD"), label: s("patron password") });
        f.insert(s("personal_name")           , F { code: s("AE"), label: s("personal name") });
        f.insert(s("screen_msg")              , F { code: s("AF"), label: s("screen message") });
        f.insert(s("print_line")              , F { code: s("AG"), label: s("print line") });
        f.insert(s("due_date")                , F { code: s("AH"), label: s("due date") });
        f.insert(s("title_ident")             , F { code: s("AJ"), label: s("title identifier") });
        f.insert(s("blocked_card_msg")        , F { code: s("AL"), label: s("blocked card msg") });
        f.insert(s("library_name")            , F { code: s("AM"), label: s("library name") });
        f.insert(s("terminal_location")       , F { code: s("AN"), label: s("terminal location") });
        f.insert(s("institution_id")          , F { code: s("AO"), label: s("institution id") });
        f.insert(s("current_location")        , F { code: s("AP"), label: s("current location") });
        f.insert(s("permanent_location")      , F { code: s("AQ"), label: s("permanent location") });
        f.insert(s("hold_items")              , F { code: s("AS"), label: s("hold items") });
        f.insert(s("overdue_items")           , F { code: s("AT"), label: s("overdue items") });
        f.insert(s("charged_items")           , F { code: s("AU"), label: s("charged items") });
        f.insert(s("fine_items")              , F { code: s("AV"), label: s("fine items") });
        f.insert(s("sequence_number")         , F { code: s("AY"), label: s("sequence number") });
        f.insert(s("checksum")                , F { code: s("AZ"), label: s("checksum") });
        f.insert(s("home_address")            , F { code: s("BD"), label: s("home address") });
        f.insert(s("email_address")           , F { code: s("BE"), label: s("e-mail address") });
        f.insert(s("home_phone")              , F { code: s("BF"), label: s("home phone number") });
        f.insert(s("owner")                   , F { code: s("BG"), label: s("owner") });
        f.insert(s("currency")                , F { code: s("BH"), label: s("currency type") });
        f.insert(s("cancel")                  , F { code: s("BI"), label: s("cancel") });
        f.insert(s("transaction_id")          , F { code: s("BK"), label: s("transaction id") });
        f.insert(s("valid_patron")            , F { code: s("BL"), label: s("valid patron") });
        f.insert(s("renewed_items")           , F { code: s("BM"), label: s("renewed items") });
        f.insert(s("unrenewed_items")         , F { code: s("BN"), label: s("unrenewed items") });
        f.insert(s("fee_acknowleged")         , F { code: s("BO"), label: s("fee acknowledged") });
        f.insert(s("start_item")              , F { code: s("BP"), label: s("start item") });
        f.insert(s("end_item")                , F { code: s("BQ"), label: s("end item") });
        f.insert(s("queue_position")          , F { code: s("BR"), label: s("queue position") });
        f.insert(s("pickup_location")         , F { code: s("BS"), label: s("pickup location") });
        f.insert(s("recall_items")            , F { code: s("BU"), label: s("recall items") });
        f.insert(s("fee_type")                , F { code: s("BT"), label: s("fee type") });
        f.insert(s("fee_limit")               , F { code: s("CC"), label: s("fee limit") });
        f.insert(s("fee_amount")              , F { code: s("BV"), label: s("fee amount") });
        f.insert(s("expire_date")             , F { code: s("BW"), label: s("expiration date") });
        f.insert(s("supported_messages")      , F { code: s("BX"), label: s("supported messages") });
        f.insert(s("hold_type")               , F { code: s("BY"), label: s("hold type") });
        f.insert(s("hold_items_limit")        , F { code: s("BZ"), label: s("hold items limit") });
        f.insert(s("overdue_items_list")      , F { code: s("CA"), label: s("overdue items limit") });
        f.insert(s("charged_items_limit")     , F { code: s("CB"), label: s("charged items limit") });
        f.insert(s("unavail_hold_items")      , F { code: s("CD"), label: s("unavailable hold items") });
        f.insert(s("hold_queue_length")       , F { code: s("CF"), label: s("hold queue length") });
        f.insert(s("fee_identifier")          , F { code: s("CG"), label: s("fee identifier") });
        f.insert(s("item_properties")         , F { code: s("CH"), label: s("item properties") });
        f.insert(s("security_inhibit")        , F { code: s("CI"), label: s("security inhibit") });
        f.insert(s("recall_date")             , F { code: s("CJ"), label: s("recall date") });
        f.insert(s("media_type")              , F { code: s("CK"), label: s("media type") });
        f.insert(s("sort_bin")                , F { code: s("CL"), label: s("sort bin") });
        f.insert(s("hold_pickup_date")        , F { code: s("CM"), label: s("hold pickup date") });
        f.insert(s("login_user_id")           , F { code: s("CN"), label: s("login user id") });
        f.insert(s("location_code")           , F { code: s("CP"), label: s("location code") });
        f.insert(s("valid_patron_pwd")        , F { code: s("CQ"), label: s("valid patron password") });
        f.insert(s("inet_profile")            , F { code: s("PI"), label: s("patron internet profile") });
        f.insert(s("call_number")             , F { code: s("CS"), label: s("call number") });
        f.insert(s("collection_code")         , F { code: s("CR"), label: s("collection code") });
        f.insert(s("alert_type")              , F { code: s("CV"), label: s("alert type") });
        f.insert(s("hold_patron_id")          , F { code: s("CY"), label: s("hold patron id") });
        f.insert(s("hold_patron_name")        , F { code: s("DA"), label: s("hold patron name") });
        f.insert(s("dest_location")           , F { code: s("CT"), label: s("destination location") });

        // Envisionware terminal Extensions
        f.insert(s("patron_expire_date")      , F { code: s("PA"), label: s("patron expire date") });
        f.insert(s("patron_dob")              , F { code: s("PB"), label: s("patron birth date") });
        f.insert(s("patron_class")            , F { code: s("PC"), label: s("patron class") });
        f.insert(s("register_login")          , F { code: s("OR"), label: s("register login") });
        f.insert(s("check_number")            , F { code: s("RN"), label: s("check number") });
    }

    pub fn field_by_code(&self, code: &str) -> Option<&Field2> {
        for field in self.fields.values() {
            if field.code == code {
                return Some(field);
            }
        }
        None
    }

    fn set_messages(&mut self) {

        let m = &mut self.messages; // local shorthand
        type M = Message2;          // local shorthand

        let msg = M {
            code: s("99"),
            label: s("SC Status"),
            fixed_fields: Vec::new()
        };

        msg.fixed_fields.push(self.fixed_field_by_code("status_code").unwrap());

        m.insert(s("sc_status"), msg);
    }

/*
pub const M_SC_STATUS: Message = Message {
    code: "99",
    label: "SC Status",
    fixed_fields: &[
        &FF_STATUS_CODE,
        &FF_MAX_PRINT_WIDTH,
        &FF_PROTOCOL_VERSION
    ],
};
*/
}


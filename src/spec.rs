use super::error::Error;
use std::fmt;

pub const SIP_PROTOCOL_VERSION: &str = "2.00";

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


// To make the below a little tidier
type FF = FixedField;

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
pub const FF_UNAVAIL_ITEMS_COUNT : FF = FF { length: 4,  label: "unavail holds count" };
pub const FF_RENEWAL_POLICY      : FF = FF { length: 1,  label: "sc renewal policy" };
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

type F = Field;

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
pub const F_UNAVAILABLE_HOLD_ITEMS  : F = F { code: "CD", label: "unavailable hold items" };
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




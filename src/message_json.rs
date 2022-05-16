use super::error::Error;
use std::error;
use std::fmt;
use super::spec;
use super::Message;
use super::FixedField;
use super::Field;
use std::collections::HashMap;
use serde_json as json;

/// Errors related specifically to SIP <=> JSON routines
#[derive(Debug)]
pub enum SipJsonError {
    MessageFormatError(String),
}

use SipJsonError::*;

impl fmt::Display for SipJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageFormatError(s) =>
                write!(f, "SIP message could not be translated to/from JSON: {}", s),
        }
    }
}

impl Message {

    /// Translate a SIP Message into a JSON object.
    ///
    /// ```
    /// use sip2::{Message, Field, FixedField};
    /// use sip2::spec;
    /// use serde_json as json;
    ///
    /// let msg = Message::new(
    ///     &spec::M_LOGIN,
    ///     vec![
    ///         FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
    ///         FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
    ///     ],
    ///     vec![
    ///         Field::new(spec::F_LOGIN_UID.code, "sip_username"),
    ///         Field::new(spec::F_LOGIN_PWD.code, "sip_password"),
    ///     ]
    /// );
    ///
    /// let json_val = msg.to_json_value().unwrap();
    /// let expected = json::json!({
    ///   "code":"93",
    ///   "fixed_fields":["0","0"],
    ///   "fields":[{"CN":"sip_username"},{"CO":"sip_password"}]});
    ///
    /// assert_eq!(expected, json_val);
    /// ```
    pub fn to_json_value(&self) -> Result<serde_json::Value, SipJsonError> {

        let ff: Vec<String> = self.fixed_fields().iter()
            .map(|f| f.value().to_string()).collect();

        let mut fields: Vec<HashMap<String, String>> = Vec::new();

        for f in self.fields().iter() {
            let mut map = HashMap::new();
            map.insert(f.code().to_string(), f.value().to_string());
            fields.push(map);
        }

        Ok(json::json!({
            "code": self.spec().code,
            "fixed_fields": ff,
            "fields": fields
        }))
    }

    pub fn to_json(&self) -> Result<String, SipJsonError> {
        match self.to_json_value() {
            Ok(jv) => {
                match json::to_string(&jv) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(SipJsonError::MessageFormatError(format!("{}", e))),
                }
            },
            Err(e) => Err(e)
        }
    }

    /// Translate a JSON object into a SIP Message.
    ///
    /// ```
    /// use sip2::{Message, Field, FixedField};
    /// use sip2::spec;
    /// use serde_json as json;
    ///
    /// let expected = Message::new(
    ///     &spec::M_LOGIN,
    ///     vec![
    ///         FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
    ///         FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
    ///     ],
    ///     vec![
    ///         Field::new(spec::F_LOGIN_UID.code, "sip_username"),
    ///         Field::new(spec::F_LOGIN_PWD.code, "sip_password"),
    ///     ]
    /// );
    ///
    /// let json_val = json::json!({
    ///   "code":"93",
    ///   "fixed_fields":["0","0"],
    ///   "fields":[{"CN":"sip_username"},{"CO":"sip_password"}]});
    ///
    /// let msg = Message::from_json_value(&json_val).unwrap();
    ///
    /// assert_eq!(expected, msg);
    /// ```
    pub fn from_json_value(json_value: &json::Value) -> Result<Message, SipJsonError> {

        let msg_code = match &json_value["code"] {
            json::Value::String(c) => c,
            _ => {
                return Err(
                    SipJsonError::MessageFormatError(
                        format!("Missing 'code' value")));
            }
        };

        let msg_spec = match spec::Message::from_code(&msg_code) {
            Some(s) => s,
            _ => {
                return Err(SipJsonError::MessageFormatError(
                    format!("Invalid SIP message code: {}", msg_code)));
            }
        };

        let mut fixed_fields: Vec<FixedField> = Vec::new();

        if let json::Value::Array(json_arr) = &json_value["fixed_fields"] {
            let mut idx = 0;

            for ff_spec in msg_spec.fixed_fields.iter() {

                if let Some(json_val) = json_arr.get(idx) {
                    if let Some(val) = json_val.as_str() {
                        if let Ok(ff) = FixedField::new(ff_spec, &val) {
                            fixed_fields.push(ff);
                        }
                    }
                }

                idx += 1;
            }
        }

        // {"AO": "institution name"}
        let mut fields: Vec<Field> = Vec::new();

        let mut err = false;
        if let json::Value::Array(json_arr) = &json_value["fields"] {
            for json_field in json_arr.iter() {

                if let Some(json_hash) = json_field.as_object() {
                    if let Some(code) = json_hash.keys().next() {
                        if code.len() == 2 {
                            if let Some(value) = json_hash[code].as_str() {
                                fields.push(Field::new(code, &value));
                            } else { err = true; }
                        }
                    } else { err = true; }

                } else { err = true; }
            }

        } // "fields" not required for all messages

        if err {
            return Err(SipJsonError::MessageFormatError(
                format!("'fields' array contains invalid fields")));
        }

        Ok(Message::new(msg_spec, fixed_fields, fields))
    }

    /// Translate a JSON string into a SIP Message.
    ///
    /// ```
    /// use sip2::{Message, Field, FixedField};
    /// use sip2::spec;
    /// use serde_json as json;
    ///
    /// let expected = Message::new(
    ///     &spec::M_LOGIN,
    ///     vec![
    ///         FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
    ///         FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
    ///     ],
    ///     vec![
    ///         Field::new(spec::F_LOGIN_UID.code, "sip_username"),
    ///         Field::new(spec::F_LOGIN_PWD.code, "sip_password"),
    ///     ]
    /// );
    ///
    /// let json_str = r#"
    ///   {
    ///     "code":"93",
    ///     "fixed_fields":["0","0"],
    ///     "fields":[{"CN":"sip_username"},{"CO":"sip_password"}]
    ///   }
    /// "#;
    ///
    /// let msg = Message::from_json(&json_str).unwrap();
    ///
    /// assert_eq!(expected, msg);
    /// ```

    pub fn from_json(msg_json: &str) -> Result<Message, SipJsonError> {

        let json_value: json::Value = match json::from_str(msg_json) {
            Ok(v) => v,
            Err(e) => {
                return Err(SipJsonError::MessageFormatError(format!("{}", e)));
            }
        };

        Message::from_json_value(&json_value)
    }
}


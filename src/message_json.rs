use super::error::Error;
use std::error;
use std::fmt;
use super::spec;
use super::Message;
use super::FixedField;
use super::Field;
use std::collections::HashMap;
use serde_json as json;

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

    pub fn to_json(msg: &Message) -> Result<String, SipJsonError> {

        let ff: Vec<String> = msg.fixed_fields().iter()
            .map(|f| f.value().to_string()).collect();

        let mut fields: Vec<HashMap<String, String>> = Vec::new();

        for f in msg.fields().iter() {
            let mut map = HashMap::new();
            map.insert(f.code().to_string(), f.value().to_string());
            fields.push(map);
        }

        let json_value = json::json!({
            "code": msg.spec().code,
            "fixed_fields": ff,
            "fields": fields
        });

        match json::to_string(&json_value) {
            Ok(s) => Ok(s),
            Err(e) => Err(SipJsonError::MessageFormatError(format!("{}", e)))
        }
    }

    pub fn from_json(msg_json: &str) -> Result<Message, SipJsonError> {

        let json_value: json::Value = match json::from_str(msg_json) {
            Ok(v) => v,
            Err(e) => {
                return Err(SipJsonError::MessageFormatError(format!("{}", e)));
            }
        };

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
                    if json_val.is_string() {
                        if let Ok(ff) = FixedField::new(ff_spec, &json_val.to_string()) {
                            fixed_fields.push(ff);
                        }
                    }
                }

                idx += 1;
            }
        }

        // TODO confirm the fixed fields add up to the correct length

        // {"AO": "institution name"}
        let mut fields: Vec<Field> = Vec::new();

        if let json::Value::Array(json_arr) = &json_value["fields"] {
            for json_field in json_arr.iter() {
                if let Some(json_hash) = json_field.as_object() {
                    if let Some(code) = json_hash.keys().next() {
                        if code.len() == 2 {
                            if let Some(value) = json_hash[code].as_str() {
                                fields.push(Field::new(code, &value));
                            }
                        }
                    }
                }
            }
        }

        Ok(Message::new(msg_spec, fixed_fields, fields))
    }
}


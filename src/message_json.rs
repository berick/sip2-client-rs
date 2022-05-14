use super::error::Error;
use super::spec;
use super::Message;
use super::FixedField;
use super::Field;
use std::collections::HashMap;
use serde_json as json;

// TODO
// More appropriate error handling with failure info strings

impl Message {

    pub fn to_json(msg: &Message) -> Result<String, Error> {

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
            _ => Err(Error::MessageFormatError)
        }
    }

    pub fn from_json(msg_json: &str) -> Result<Message, Error> {

        let json_value: json::Value = match json::from_str(msg_json) {
            Ok(v) => v,
            _ => { return Err(Error::MessageFormatError); }
        };

        let msg_code = match &json_value["code"] {
            json::Value::String(c) => c,
            _ => { return Err(Error::MessageFormatError); }
        };

        let msg_spec = match spec::Message::from_code(&msg_code) {
            Some(s) => s,
            _ => { return Err(Error::MessageFormatError); }
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

        Ok(Message::new(msg_spec, fixed_fields, vec![]))
    }
}


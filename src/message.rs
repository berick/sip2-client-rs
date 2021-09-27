use std::fmt;
use super::error;
use super::spec;
use super::util;

pub struct FixedField {
    spec: &'static spec::FixedField,
    value: String
}

impl FixedField {

    pub fn new(spec: &'static spec::FixedField, value: &str) -> Result<Self, error::Error> {
        if value.len() == spec.length.into() {
            Ok(FixedField {
                spec: spec,
                value: value.to_string(),
            })
        } else {
            Err(error::Error::FixedFieldLengthError)
        }
    }

    pub fn spec(&self) -> &'static spec::FixedField {
        self.spec
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    /// Translate a FixedField into a string which can be inserted into
    /// a SIP message.
    ///
    /// ```
    /// use sip2::FixedField;
    /// use sip2::spec;
    /// let ff = FixedField::new(&spec::FF_MAX_PRINT_WIDTH, "999").unwrap();
    /// assert_eq!(ff.to_sip(), "999");
    /// ```
    pub fn to_sip(&self) -> String {
        util::sip_string(&self.value)
    }
}

impl fmt::Display for FixedField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   {}{:.>25}", self.spec.label, self.value)
    }
}

pub struct Field {
    spec: &'static spec::Field,
    value: String
}

impl Field {

    pub fn new(spec: &'static spec::Field, value: &str) -> Self {
        Field {
            spec,
            value: value.to_string(),
        }
    }

    pub fn spec(&self) -> &'static spec::Field {
        self.spec
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    /// Create a SIP string from a field
    ///
    /// ```
    /// use sip2::Field;
    /// use sip2::spec;
    /// let f = Field::new(&spec::F_LOGIN_UID, "sip_username");
    /// assert_eq!(f.to_sip(), "CNsip_username|");
    /// ```
    pub fn to_sip(&self) -> String {
        self.spec.code.to_string() + &util::sip_string(&self.value) + &String::from("|")
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:.<37}{}", self.spec.code, self.spec.label, self.value)
    }
}

pub struct Message {
    spec: &'static spec::Message,
    fields: Vec<Field>,
    fixed_fields: Vec<FixedField>,
}

impl Message {

    pub fn new(spec: &'static spec::Message,
        fixed_fields: Vec<FixedField>, fields: Vec<Field>) -> Self {

        Message {
            spec,
            fixed_fields,
            fields,
        }
    }

    pub fn spec(&self) -> &'static spec::Message {
        self.spec
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn fixed_fields(&self) -> &Vec<FixedField> {
        &self.fixed_fields
    }

    /// Create a SIP string of a message.
    ///
    /// ```
    /// use sip2::{Message, Field, FixedField};
    /// use sip2::spec;
    ///
    /// let msg = Message::new(
    ///     &spec::M_LOGIN,
    ///     vec![
    ///         FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
    ///         FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
    ///     ],
    ///     vec![
    ///         Field::new(&spec::F_LOGIN_UID, "sip_username"),
    ///         Field::new(&spec::F_LOGIN_PWD, "sip_password"),
    ///     ]
    /// );
    ///
    /// assert_eq!(msg.to_sip(), "9300CNsip_username|COsip_password|");
    /// ```
    pub fn to_sip(&self) -> String {
        let mut s = self.spec.code.to_string();

        for ff in self.fixed_fields.iter() {
            s.push_str(&ff.to_sip());
        }

        for f in self.fields.iter() {
            s.push_str(&f.to_sip());
        }

        s
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\n", self.spec.code, self.spec.label);
        for ff in self.fixed_fields.iter() {
            write!(f, "{}\n", ff);
        }
        for field in self.fields.iter() {
            write!(f, "{}\n", field);
        }
        write!(f, "")
    }
}

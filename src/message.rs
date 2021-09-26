use super::spec;
use super::util;
use super::util::Util;

pub struct FixedField {
    spec: &'static spec::FixedField,
    value: String
}

impl FixedField {

    /// Use this new() when the length of the value is known to be correct
    pub fn new(spec: &'static spec::FixedField, value: &str) -> Self {
        FixedField {
            spec,
            value: value.to_string(),
        }
    }

    /// Use this when the length of the value cannot be anticipated at runtime.
    pub fn new_checked(spec: &'static spec::FixedField, value: &str) -> Option<Self> {
        if value.len() != spec.length.into() { return None; }
        Some(Self::new(spec, value))
    }

    pub fn spec(&self) -> &'static spec::FixedField {
        self.spec
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn to_sip(&self) -> String {
        Util::sip_string(&self.value)
    }

    pub fn to_str(&self) -> String {
        format!("{}{: >35}", self.spec.label, self.value)
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

    pub fn to_sip(&self) -> String {
        self.spec.code.to_string() + &Util::sip_string(&self.value) + &String::from("|")
    }

    pub fn to_str(&self) -> String {
        let s = format!("{}{}", self.spec.code, self.spec.label);
        format!("{: <35}{}", s, self.value)
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

    pub fn to_str(&self) -> String {
        String::from("") // TODO
    }
}


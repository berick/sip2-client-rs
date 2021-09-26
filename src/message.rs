use super::spec;

pub struct FixedField {
    spec: &'static spec::FixedField,
    value: String
}

impl FixedField {

    pub fn new(spec: &'static spec::FixedField, value: &str) -> Self {
        FixedField {
            spec,
            value: value.to_string(),
        }
    }

    /// Creates a new FixedField IF the value provided is the correct length
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
        String::from("") // TODO
    }

    pub fn to_str(&self) -> String {
        String::from("") // TODO
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
        String::from("") // TODO
    }

    pub fn to_str(&self) -> String {
        String::from("") // TODO
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
        String::from("") // TODO
    }

    pub fn to_str(&self) -> String {
        String::from("") // TODO
    }
}


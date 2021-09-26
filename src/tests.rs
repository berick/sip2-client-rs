use super::spec;
use super::message::FixedField;
use super::message::Field;
use super::message::Message;

#[test]
fn invalid_fixed_field() {
    assert_eq!(
        FixedField::new_checked(&spec::FF_STATUS_CODE, "123").is_none(),
        true
    );
}

#[test]
fn message_to_sip() {

    let msg = Message::new(
        &spec::M_SC_STATUS,
        vec![
            FixedField::new(&spec::FF_STATUS_CODE, " "),
            FixedField::new(&spec::FF_MAX_PRINT_WIDTH, "111"),
            FixedField::new(&spec::FF_PROTOCOL_VERSION, "1111"),
        ],
        vec![]
    );

    assert_eq!(msg.spec().fixed_fields[0].length, 1);

    // TODO test to_sip
}


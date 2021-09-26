/*
use self::error;
use self::spec;
use self::message;
*/

mod error;
mod spec;
mod message;

#[cfg(test)]
mod tests {
    #[test]

    fn create_message() {
        use super::spec;
        use super::message::FixedField;
        use super::message::Field;
        use super::message::Message;


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
        assert_eq!(msg.spec().fixed_fields[1].length, 3);
        assert_eq!(msg.spec().fixed_fields[2].length, 4);

        assert_eq!(2 + 2, 4);
    }
}

pub use self::error::Error;
pub use self::message::FixedField;
pub use self::message::Field;
pub use self::message::Message;

mod error;
mod spec;
mod message;

#[cfg(test)]
mod tests;


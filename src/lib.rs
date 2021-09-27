pub use self::error::Error;
pub use self::message::FixedField;
pub use self::message::Field;
pub use self::message::Message;

// These are mostly used internally
pub mod spec;
pub mod util;

mod error;
mod message;

#[cfg(test)]
mod tests;


pub use self::error::Error;
pub use self::message::FixedField;
pub use self::message::Field;
pub use self::message::Message;
pub use self::connection::Connection;

pub use self::client::Client;
pub use self::client::LoginParams;
pub use self::client::PatronStatusParams;
pub use self::client::PatronInfoParams;
pub use self::client::ItemInfoParams;

pub mod spec;
pub mod util;

mod error;
mod message;
mod connection;
mod client;

#[cfg(test)]
mod tests;


#[cfg(test)]
mod tests;

mod parser;
mod types;

use super::error::Error;
use parser::ValueParser;

pub use types::*;

/// A Bencoded value
#[derive(Debug, Clone)]
pub enum Value {
    Integer(Integer),
    ByteString(ByteString),
    List(List),
    Dictionary(Dictionary),
}

impl Value {
    /// Parse Bencode from a slice of bytes.
    pub fn from_bytes(contents: &[u8]) -> Result<Self, Error> {
        ValueParser::with(contents, 0).parse()
    }

    pub fn try_as<T: TryFrom<Value>>(self) -> Result<T, Error> {
        T::try_from(self).map_err(|_| Error::Bencode("wrong type".to_string()))
    }

    pub fn as_list_of<T: TryFrom<Value>>(self) -> Result<Vec<T>, Error> {
        self.try_as::<List>()?
            .0
            .into_iter()
            .map(|v| v.try_as::<T>())
            .collect::<Result<Vec<T>, Error>>()
    }
}

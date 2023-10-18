use crate::prelude::*;
use std::collections::BTreeMap;

/// (Bencode) Encoder.
pub struct Encoder<'a> {
    value: &'a Value,
}

impl<'a> Encoder<'a> {
    /// Create a new `Encoder`.
    pub fn with(value: &'a Value) -> Self {
        Self { value }
    }

    /// Parse any.
    pub fn parse(&self) -> Vec<u8> {
        Self::parse_any(self.value)
    }

    /// Parse any.
    fn parse_any(value: &Value) -> Vec<u8> {
        match value {
            Value::Integer(inner) => Self::parse_integer(&inner.0),
            Value::ByteString(inner) => Self::parse_byte_string(&inner.0),
            Value::List(inner) => Self::parse_list(&inner.0),
            Value::Dictionary(inner) => Self::parse_dictionary(&inner.0),
        }
    }

    /// Parse integer.
    fn parse_integer(data: &isize) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(b"i"[0]);
        out.append(&mut data.to_string().as_bytes().to_vec());
        out.push(b"e"[0]);

        out
    }

    /// Parse byte string.
    fn parse_byte_string(data: &Vec<u8>) -> Vec<u8> {
        let mut out = Vec::with_capacity(data.len());
        out.append(&mut data.len().to_string().as_bytes().to_vec());
        out.push(b":"[0]);
        out.append(&mut data.clone());

        out
    }

    /// Parse list.
    fn parse_list(data: &Vec<Value>) -> Vec<u8> {
        let mut out = Vec::with_capacity(data.len());
        out.push(b"l"[0]);
        for value in data {
            out.append(&mut Self::parse_any(value));
        }
        out.push(b"e"[0]);

        out
    }

    /// Parse dictionary.
    fn parse_dictionary(data: &BTreeMap<ByteString, Value>) -> Vec<u8> {
        let mut out = Vec::with_capacity(data.len());
        out.push(b"d"[0]);
        for (key, value) in data {
            out.append(&mut Self::parse_byte_string(&key.0));
            out.append(&mut Self::parse_any(value));
        }
        out.push(b"e"[0]);

        out
    }
}

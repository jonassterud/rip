use crate::error::Error;
use crate::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Integer(pub isize);

impl TryFrom<Value> for Integer {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(out) => Ok(out),
            _ => Err(Error::Bencode("not a integer".to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ByteString(pub Vec<u8>);

impl TryFrom<Value> for ByteString {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::ByteString(out) => Ok(out),
            _ => Err(Error::Bencode("not a byte string".to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct List(pub Vec<Value>);

impl TryFrom<Value> for List {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::List(out) => Ok(out),
            _ => Err(Error::Bencode("not a list".to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dictionary(pub BTreeMap<ByteString, Value>);

impl TryFrom<Value> for Dictionary {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Dictionary(out) => Ok(out),
            _ => Err(Error::Bencode("not a dictionary".to_string())),
        }
    }
}

impl Dictionary {
    pub fn try_get(&self, key: &str) -> Result<&Value, Error> {
        let opt_value = self.0.get(&ByteString(key.as_bytes().to_vec()));
        let out_value = opt_value.ok_or(Error::Bencode(format!("missing key {key:?}")))?;

        Ok(out_value)
    }

    pub fn try_get_as<T: TryFrom<Value>>(&self, key: &str) -> Result<T, Error> {
        let opt_value = self.0.get(&ByteString(key.as_bytes().to_vec()));
        let res_value = opt_value.ok_or(Error::Bencode(format!("missing key {key:?}")))?;
        let res_value = res_value.clone();
        let out_value = T::try_from(res_value).map_err(|_| Error::Bencode("".to_string()))?;

        Ok(out_value)
    }

    pub fn has(&self, key: &str) -> bool {
        self.0.contains_key(&ByteString(key.as_bytes().to_vec()))
    }
}

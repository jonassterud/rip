use super::error::Error;
use std::collections::BTreeMap;
use std::ops::Range;

#[cfg(test)]
mod tests;

/// A Bencoded value
#[derive(Debug, Clone)]
pub enum Value {
    Integer(isize),
    ByteString(Vec<u8>),
    List(Vec<Value>),
    Dictionary(BTreeMap<Vec<u8>, Value>),
}

pub struct ValueParser<'a> {
    data: &'a [u8],
    i: usize,
}

impl Value {
    /// Parse Bencode from a slice of bytes.
    pub fn from_bytes(contents: &[u8]) -> Result<Self, Error> {
        ValueParser {
            data: contents,
            i: 0,
        }
        .parse()
    }

    /// Try to get `Value` as a integer.
    pub fn as_integer(self) -> Result<isize, Error> {
        match self {
            Value::Integer(integer) => Ok(integer),
            _ => Err(Error::Bencode("not a integer".into())),
        }
    }

    /// Try to get `Value` as a byte string.
    pub fn as_byte_string(self) -> Result<Vec<u8>, Error> {
        match self {
            Value::ByteString(byte_string) => Ok(byte_string),
            _ => Err(Error::Bencode("not a byte string".into())),
        }
    }

    /// Try to get `Value` as a list.
    pub fn as_list(self) -> Result<Vec<Value>, Error> {
        match self {
            Value::List(list) => Ok(list),
            _ => Err(Error::Bencode("not a list".into())),
        }
    }

    /// Try to get `Value` as a dictionary.
    pub fn as_dictionary(self) -> Result<BTreeMap<Vec<u8>, Value>, Error> {
        match self {
            Value::Dictionary(dictionary) => Ok(dictionary),
            _ => Err(Error::Bencode("not a dictionary".into())),
        }
    }

    /// Try to get `Value` as a list of byte strings.
    pub fn as_list_of_byte_strings(self) -> Result<Vec<Vec<u8>>, Error> {
        match self {
            Value::List(list) => list.into_iter().map(|v| v.as_byte_string()).collect(),
            _ => Err(Error::Bencode("not a list".into())),
        }
    }

    pub fn as_list_of_dictionaries(&self) -> Result<Vec<BTreeMap<Vec<u8>, Value>>, Error> {
        match self {
            Value::List(list) => list.iter().cloned().map(|v| v.as_dictionary()).collect(),
            _ => Err(Error::Bencode("not a list".into())),
        }
    }
}

impl<'a> ValueParser<'a> {
    /// Get current byte at index.
    fn at(&self) -> Result<&u8, Error> {
        self.data.get(self.i).ok_or(Error::Bencode("empty".into()))
    }

    /// Skip n bytes.
    fn skip(&mut self, n: usize) {
        self.i += n;
    }

    /// Find index of byte from index.
    fn find(&self, byte: u8) -> Result<usize, Error> {
        for j in self.i..self.data.len() {
            if self.data[j] == byte {
                return Ok(j - self.i);
            }
        }

        Err(Error::Bencode("not found".into()))
    }

    /// Take bytes in the range, relative to the index.
    fn take(&mut self, range: Range<usize>) -> Result<&[u8], Error> {
        match self.data.get(self.i + range.start..self.i + range.end) {
            Some(out) => {
                self.i += range.end;
                Ok(out)
            }
            _ => Err(Error::Bencode("out of range".into())),
        }
    }

    /// Parse any.
    pub fn parse(&mut self) -> Result<Value, Error> {
        match self.at()? {
            b'i' => self.parse_integer(),
            48..=57 => self.parse_byte_string(),
            b'l' => self.parse_list(),
            b'd' => self.parse_dictionary(),
            _ => Err(Error::Bencode("unexpected byte".into())),
        }
    }

    /// Parse integer.
    fn parse_integer(&mut self) -> Result<Value, Error> {
        let end = self.find(b'e')?;
        let val = self
            .take(1..end)?
            .iter()
            .map(|b| *b as char)
            .collect::<String>();
        let val = val
            .parse::<isize>()
            .map_err(|_| Error::Bencode("invalid integer".into()))?;
        self.skip(1);

        Ok(Value::Integer(val))
    }

    /// Parse byte string.
    fn parse_byte_string(&mut self) -> Result<Value, Error> {
        let end = self.find(b':')?;
        let len = self
            .take(0..end)?
            .iter()
            .map(|b| *b as char)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| Error::Bencode("invalid length".into()))?;
        let val = self.take(1..(len + 1))?.to_vec();

        Ok(Value::ByteString(val))
    }

    /// Parse list.
    fn parse_list(&mut self) -> Result<Value, Error> {
        let mut val = Vec::new();

        self.skip(1);
        while *self.at()? != b'e' {
            val.push(self.parse()?);
        }
        self.skip(1);

        Ok(Value::List(val))
    }

    /// Parse dictionary.
    fn parse_dictionary(&mut self) -> Result<Value, Error> {
        let mut val = BTreeMap::new();

        self.skip(1);
        while *self.at()? != b'e' {
            let key = self.parse_byte_string()?.as_byte_string()?;
            let value = self.parse()?;
            val.insert(key, value);
        }
        self.skip(1);

        Ok(Value::Dictionary(val))
    }
}

use crate::error::Error;
use crate::prelude::*;
use std::collections::BTreeMap;
use std::ops::Range;

/// (Bencode) Decoder
pub struct Decoder<'a> {
    data: &'a [u8],
    i: usize,
}

impl<'a> Decoder<'a> {
    /// Create a new `Decoder`.
    pub fn with(data: &'a [u8], i: usize) -> Self {
        Self { data, i }
    }

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
            b'i' => Ok(Value::Integer(self.parse_integer()?)),
            48..=57 => Ok(Value::ByteString(self.parse_byte_string()?)),
            b'l' => Ok(Value::List(self.parse_list()?)),
            b'd' => Ok(Value::Dictionary(self.parse_dictionary()?)),
            _ => Err(Error::Bencode("unexpected byte".into())),
        }
    }

    /// Parse integer.
    fn parse_integer(&mut self) -> Result<Integer, Error> {
        let end = self.find(b'e')?;
        let val = self
            .take(1..end)?
            .iter()
            .map(|b| *b as char)
            .collect::<String>();
        if val.starts_with("0") && val.len() > 1 {
            return Err(Error::Bencode("invalid integer".to_string()));
        }
        let val = val
            .parse::<isize>()
            .map_err(|_| Error::Bencode("invalid integer".into()))?;
        self.skip(1);

        Ok(Integer(val))
    }

    /// Parse byte string.
    fn parse_byte_string(&mut self) -> Result<ByteString, Error> {
        let end = self.find(b':')?;
        let len = self
            .take(0..end)?
            .iter()
            .map(|b| *b as char)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| Error::Bencode("invalid length".into()))?;
        let val = self.take(1..(len + 1))?.to_vec();

        Ok(ByteString(val))
    }

    /// Parse list.
    fn parse_list(&mut self) -> Result<List, Error> {
        let mut val = Vec::new();

        self.skip(1);
        while *self.at()? != b'e' {
            val.push(self.parse()?);
        }
        self.skip(1);

        Ok(List(val))
    }

    /// Parse dictionary.
    fn parse_dictionary(&mut self) -> Result<Dictionary, Error> {
        let mut val = BTreeMap::new();

        self.skip(1);
        while *self.at()? != b'e' {
            let key = self.parse_byte_string()?;
            let value = self.parse()?;
            val.insert(key, value);
        }
        self.skip(1);

        Ok(Dictionary(val))
    }
}

#[test]
fn test_value_parser() {
    let mut parser = Decoder::with(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0);

    assert_eq!(parser.at().unwrap(), &1);
    assert_eq!(parser.take(0..4).unwrap(), &[1, 2, 3, 4]);
    assert_eq!(parser.find(5).unwrap(), 0);
    assert_eq!(parser.find(10).unwrap(), 5);
    assert_eq!(parser.take(1..2).unwrap(), &[6]);
}

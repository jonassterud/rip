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
        self.data
            .get(self.i)
            .ok_or(Error::Bencode(format!("slice is empty")))
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

        Err(Error::Bencode(format!("byte ({byte}) not found")))
    }

    /// Take bytes in the range, relative to the index.
    fn take(&mut self, range: Range<usize>) -> Result<&[u8], Error> {
        match self.data.get(self.i + range.start..self.i + range.end) {
            Some(out) => {
                self.i += range.end;
                Ok(out)
            }
            _ => Err(Error::Bencode(format!("out of range"))),
        }
    }

    /// Parse any.
    pub fn parse(&mut self) -> Result<Value, Error> {
        let byte = *self.at()?;

        match byte {
            b'i' => Ok(Value::Integer(self.parse_integer()?)),
            48..=57 => Ok(Value::ByteString(self.parse_byte_string()?)),
            b'l' => Ok(Value::List(self.parse_list()?)),
            b'd' => Ok(Value::Dictionary(self.parse_dictionary()?)),
            _ => Err(Error::Bencode(format!("unexpected byte ({byte})"))),
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
            return Err(Error::Bencode(format!(
                "invalid integer (stopped at \"{val}\")"
            )));
        }
        let val = val
            .parse::<isize>()
            .map_err(|_| Error::Bencode(format!("invalid integer (stopped at \"{val}\"")))?;
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
            .collect::<String>();
        let len = len.parse::<usize>().map_err(|_| {
            Error::Bencode(format!(
                "failed parsing length of byte string (stopped at \"{len}\"",
            ))
        })?;
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

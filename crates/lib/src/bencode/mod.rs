use super::error::Error;
use core::num;
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

impl Value {
    /// Parse Bencode from a slice of bytes.
    pub fn from_bytes(contents: &[u8]) -> Result<Self, Error> {
        Value::parse_any(contents, &mut 0)
    }

    /// Try to get `Value` as a integer.
    pub fn try_as_integer(self) -> Result<isize, Error> {
        match self {
            Value::Integer(integer) => Ok(integer),
            _ => Err(Error::Bencode("not a integer".into())),
        }
    }

    /// Try to get `Value` as a byte string.
    pub fn try_as_byte_string(self) -> Result<Vec<u8>, Error> {
        match self {
            Value::ByteString(byte_string) => Ok(byte_string),
            _ => Err(Error::Bencode("not a byte string".into())),
        }
    }

    /// Try to get `Value` as a list.
    pub fn try_as_list(self) -> Result<Vec<Value>, Error> {
        match self {
            Value::List(list) => Ok(list),
            _ => Err(Error::Bencode("not a list".into())),
        }
    }

    /// Try to get `Value` as a list of byte strings.
    pub fn try_as_list_of_byte_strings(self) -> Result<Vec<Vec<u8>>, Error> {
        match self {
            Value::List(list) => list.into_iter().map(|v| v.try_as_byte_string()).collect(),
            _ => Err(Error::Bencode("not a list".into())),
        }
    }

    /// Try to get `Value` as a dictionary.
    pub fn try_as_dictionary(self) -> Result<BTreeMap<Vec<u8>, Value>, Error> {
        match self {
            Value::Dictionary(dictionary) => Ok(dictionary),
            _ => Err(Error::Bencode("not a dictionary".into())),
        }
    }

    /// Parse any data, starting at marker.
    fn parse_any(data: &[u8], marker: &mut usize) -> Result<Self, Error> {
        let prefix = data
            .get(*marker)
            .ok_or(Error::Bencode("missing prefix".into()))?;

        match prefix {
            b'i' => Self::parse_integer(data, marker),
            48..=57 => Self::parse_byte_string(data, marker),
            b'l' => Self::parse_list(data, marker),
            b'd' => Self::parse_dictionary(data, marker),
            _ => Err(Error::Bencode("unknown prefix".into())),
        }
    }

    /// Parse integer, starting at marker.
    fn parse_integer(data: &[u8], marker: &mut usize) -> Result<Self, Error> {
        let number_range = Self::range_between(data, b'i', b'e', marker)?;
        let number = data[number_range]
            .iter()
            .map(|b| *b as char)
            .collect::<String>()
            .parse()
            .map_err(|_| Error::Bencode("failed parsing".into()))?;

        Ok(Value::Integer(number))
    }

    /// Parse byte string, starting at marker.
    fn parse_byte_string(data: &[u8], marker: &mut usize) -> Result<Self, Error> {
        let number_range = Self::range_between(data, data[*marker], b':', marker)?;
        let length = data[(number_range.start - 1)..number_range.end]
            .iter()
            .map(|b| *b as char)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| Error::Bencode("failed parsing".into()))?;
        let byte_string = &data[(number_range.end + 1)..(number_range.end + length + 1)];
        *marker += length;

        Ok(Value::ByteString(byte_string.to_vec()))
    }

    /// Parse list, starting at marker.
    fn parse_list(data: &[u8], marker: &mut usize) -> Result<Self, Error> {
        let mut list = Vec::new();

        *marker += 1;
        loop {
            if data[*marker] == b'e' {
                *marker += 1;
                return Ok(Value::List(list));
            } else {
                let next_value = Value::parse_any(data, marker)?;
                list.push(next_value)
            }
        }
    }

    /// Parse dictionary, starting at marker.
    fn parse_dictionary(data: &[u8], marker: &mut usize) -> Result<Self, Error> {
        let mut dictionary = BTreeMap::new();
        let mut key = None;

        *marker += 1;
        loop {
            if data[*marker] == b'e' {
                *marker += 1;
                return Ok(Value::Dictionary(dictionary));
            } else {
                let next_value = Value::parse_any(data, marker)?;
                if key.is_none() {
                    if let Value::ByteString(byte_string) = next_value {
                        key = Some(byte_string);
                    } else {
                        return Err(Error::Bencode("expected key".into()));
                    }
                } else {
                    dictionary.insert(key.take().unwrap(), next_value);
                }
            }
        }
    }

    /// Get `Range` for data between prefix and suffix.
    fn range_between(
        data: &[u8],
        prefix: u8,
        suffix: u8,
        marker: &mut usize,
    ) -> Result<Range<usize>, Error> {
        let mut range = Range {
            start: *marker,
            end: *marker + 1,
        };

        loop {
            let byte = data
                .get(*marker)
                .ok_or(Error::Bencode("unexpected end".into()))?;

            if *byte == prefix {
                range.start = *marker + 1;
            } else if *byte == suffix {
                range.end = *marker;
                *marker += 1;
                return Ok(range);
            }

            *marker += 1;
        }
    }
}

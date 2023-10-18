mod parser;

use crate::error::Error;
use crate::prelude::*;
use parser::Decoder;

/// Decode Bencoded data.
pub fn decode(data: &[u8]) -> Result<Value, Error> {
    Decoder::with(data, 0).parse()
}

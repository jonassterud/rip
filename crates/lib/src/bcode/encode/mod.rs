mod parser;

use crate::prelude::*;
use parser::Encoder;

/// Encode data to Bencode.
pub fn encode(data: &Value) -> Vec<u8> {
    Encoder::with(data).parse()
}

mod parser;

use crate::prelude::*;
use parser::Encoder;

/// Encode data to Bencode.
pub fn encode<T>(data: &(impl Into<T> + Clone)) -> Vec<u8>
where
    T: Into<Value>,
{
    let data: T = data.clone().into();
    let data: Value = data.into();

    Encoder::with(&data).parse()
}

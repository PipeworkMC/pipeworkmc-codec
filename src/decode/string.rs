//! `String` decoder.


use crate::decode::{
    PacketDecode,
    DecodeIter,
    IncompleteDecodeError
};
use crate::varint::{
    VarInt,
    VarIntDecodeError
};
use core::fmt::{ self, Display, Formatter };
use std::string::FromUtf8Error;


impl PacketDecode for String {
    type Error = StringDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let length = *VarInt::<u32>::decode(iter).map_err(StringDecodeError::Length)? as usize;
        let bytes  = iter.read_vec(length)?;
        let string = String::from_utf8(bytes).map_err(StringDecodeError::Utf8)?;
        Ok(string)
    }
}


/// Returned by packet decoders when a `String` was not decoded successfully.
#[derive(Debug)]
pub enum StringDecodeError {
    /// The length of the array failed to decode.
    Length(VarIntDecodeError),
    /// There were not enough bytes.
    Incomplete(IncompleteDecodeError),
    /// The decoded string was not valid UTF8.
    Utf8(FromUtf8Error)
}
impl From<IncompleteDecodeError> for StringDecodeError {
    #[inline(always)]
    fn from(err : IncompleteDecodeError) -> Self { Self::Incomplete(err) }
}
impl Display for StringDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Length(err)     => write!(f, "length {err}"),
        Self::Incomplete(err) => err.fmt(f),
        Self::Utf8(_)         => write!(f, "invalid utf8")
    } }
}

use crate::decode::{
    PacketDecode,
    DecodeBuf,
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

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let length = *VarInt::<u32>::decode(buf).map_err(StringDecodeError::Length)? as usize;
        let bytes  = buf.read_vec(length)?;
        let string = String::from_utf8(bytes).map_err(StringDecodeError::Utf8)?;
        Ok(string)
    }
}


#[derive(Debug)]
pub enum StringDecodeError {
    Length(VarIntDecodeError),
    Incomplete(IncompleteDecodeError),
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

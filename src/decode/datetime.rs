use crate::decode::{
    PacketDecode,
    DecodeBuf,
    IncompleteDecodeError
};
use chrono::{ DateTime, Utc };


impl PacketDecode for DateTime<Utc> {
    type Error = DateTimeDecodeError;
    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Self::from_timestamp(
        <_>::decode(buf).map_err(DateTimeDecodeError::Secs)?,
        <_>::decode(buf).map_err(DateTimeDecodeError::Nanos)?
    ).ok_or(DateTimeDecodeError::Invalid) }
}

pub enum DateTimeDecodeError {
    Secs(IncompleteDecodeError),
    Nanos(IncompleteDecodeError),
    Invalid
}

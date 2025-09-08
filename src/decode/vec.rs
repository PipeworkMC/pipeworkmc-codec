use crate::decode::{
    PacketDecode,
    DecodeBuf
};
use crate::varint::{
    VarInt,
    VarIntDecodeError
};
use core::fmt::{ self, Display, Formatter };


impl<T> PacketDecode for Vec<T>
where
    T : PacketDecode
{
    type Error = VecDecodeError<T::Error>;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let     length = *VarInt::<u32>::decode(buf).map_err(VecDecodeError::Length)? as usize;
        let mut vec    = Vec::with_capacity(length);
        for i in 0..length {
            vec.push(T::decode(buf).map_err(|err| VecDecodeError::Entry { index : i, err })?);
        }
        Ok(vec)
    }
}


#[derive(Debug)]
pub enum VecDecodeError<E> {
    Length(VarIntDecodeError),
    Entry {
        index : usize,
        err   : E
    }
}
impl<E> Display for VecDecodeError<E>
where
    E : Display
{
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Length(err)          => write!(f, "length {err}"),
        Self::Entry { index, err } => write!(f, "item {index} {err}")
    } }
}

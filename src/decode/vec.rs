//! `Vec<T>` decoder.


use crate::decode::{
    PacketDecode,
    DecodeIter
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

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let     length = *VarInt::<u32>::decode(iter).map_err(VecDecodeError::Length)? as usize;
        let mut vec    = Vec::with_capacity(length);
        for i in 0..length {
            vec.push(T::decode(iter).map_err(|err| VecDecodeError::Item { index : i, err })?);
        }
        Ok(vec)
    }
}


/// Returned by packet decoders when a `Vec<T>` was not decoded successfully.
#[derive(Debug)]
pub enum VecDecodeError<E> {
    /// The length of the vector failed to decode.
    Length(VarIntDecodeError),
    /// An item in the vector could not be decoded.
    Item {
        /// The index of the item that was not decoded.
        index : usize,
        /// The error.
        err   : E
    }
}
impl<E> Display for VecDecodeError<E>
where
    E : Display
{
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Length(err)          => write!(f, "length {err}"),
        Self::Item { index, err } => write!(f, "item {index} {err}")
    } }
}

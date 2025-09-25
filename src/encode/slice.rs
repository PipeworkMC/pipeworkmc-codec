//! `&[T]` encoder.


use crate::encode::{
    PacketEncode,
    EncodeBuf
};
use crate::varint::VarInt;
use core::{
    any::TypeId,
    ops::Deref
};
use std::borrow::Cow;


unsafe impl<T> PacketEncode for [T]
where
    T : PacketEncode
{

    #[inline]
    fn encode_len(&self) -> usize {
        VarInt::<u32>(self.len() as u32).encode_len()
        + self.iter().map(|item| item.encode_len()).sum::<usize>() // TODO: Special case for `[u8]`.
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        VarInt::<u32>(self.len() as u32).encode(buf);
        for item in self {
            item.encode(buf);
        }
    } }

}


unsafe impl<'l, T> PacketEncode for Cow<'l, [T]>
where
    T   : PacketEncode + 'l,
    [T] : ToOwned
{

    #[inline(always)]
    fn encode_len(&self) -> usize { <[T]>::encode_len(self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        <[T]>::encode(self, buf)
    } }

}


unsafe impl<T> PacketEncode for Vec<T>
where
    T : PacketEncode + 'static
{

    #[inline(always)]
    fn encode_len(&self) -> usize { <[T]>::encode_len(self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        <[T]>::encode(self, buf)
    } }

}


/// A `&[T]` or `Vec<T>` which will be encoded without a [`VarInt`] length.
/// Decoders must know the length from context.
#[derive(Clone, Debug)]
pub struct UnprefixedSlice<'l, T>(pub Cow<'l, [T]>)
where
    T : Clone;

impl<'l, T> From<Vec<T>> for UnprefixedSlice<'l, T>
where
    T : Clone
{
    #[inline(always)]
    fn from(value : Vec<T>) -> Self { Self(Cow::Owned(value)) }
}

impl<'l, T> Deref for UnprefixedSlice<'l, T>
where
    T : Clone
{
    type Target = [T];
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}

unsafe impl<'l, T> PacketEncode for UnprefixedSlice<'l, T>
where
    T : Clone + PacketEncode + 'static
{

    #[inline]
    fn encode_len(&self) -> usize {
        if (TypeId::of::<T>() == TypeId::of::<u8>()) {
            self.len()
        } else {
            self.iter().map(|item| item.encode_len()).sum()
        }
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        for item in &**self {
            item.encode(buf);
        }
    } }

}

use crate::{
    decode::{
        PacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    encode::{
        PacketEncode,
        EncodeBuf
    }
};
use core::{
    fmt::{ self, Display, Formatter },
    ops::Deref
};


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VarInt<T>(pub T)
where
    T : VarIntType;

impl<T> Deref for VarInt<T>
where
    T : VarIntType
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}


pub const SEGMENT_BITS : u8 = 0b01111111;
pub const CONTINUE_BIT : u8 = 0b10000000;


pub unsafe trait VarIntType
where
    Self : Copy + Sized
{

    fn decode(iter : impl Iterator<Item = u8>)
        -> Result<(Self, usize,), VarIntDecodeError>;

    type EncodeBuf : Default;

    fn encode_len(self) -> usize;

    fn encode(self, buf : &mut Self::EncodeBuf) -> &[u8];

}


macro impl_varinttype_for_signed_int($unsigned_ty:ty => $signed_ty:ty) {
    unsafe impl VarIntType for $signed_ty {

        fn decode(mut iter : impl Iterator<Item = u8>)
            -> Result<(Self, usize,), VarIntDecodeError>
        {
            const MAX_SHIFT : usize = <$signed_ty>::BITS as usize;
            let mut value    = 0;
            let mut shift    = 0;
            let mut consumed = 0;
            loop {
                let byte = iter.next().ok_or(IncompleteDecodeError)?;
                consumed += 1;
                value |= ((byte & SEGMENT_BITS) as $signed_ty) << shift;
                if ((byte & CONTINUE_BIT) == 0) { break; }
                shift += 7;
                if (shift > MAX_SHIFT) { return Err(VarIntDecodeError::TooLong); }
            }
            Ok((value, consumed,))
        }

        type EncodeBuf = [u8; size_of::<Self>() + 1];

        fn encode_len(self) -> usize {
            <$unsigned_ty as VarIntType>::encode_len(self.cast_unsigned())
        }

        fn encode(mut self, buf : &mut Self::EncodeBuf) -> &[u8] {
            const SELF_SEGMENT_BITS : $signed_ty = SEGMENT_BITS as $signed_ty;
            const SELF_CONTINUE_BIT : $signed_ty = CONTINUE_BIT as $signed_ty;
            let mut i = 0;
            loop {
                if ((self & (! SELF_SEGMENT_BITS)) == 0) {
                    *unsafe { buf.get_unchecked_mut(i) } = (self & 0xFF) as u8;
                    i += 1;
                    return &buf[0..i];
                }
                *unsafe { buf.get_unchecked_mut(i) } = ((self & SELF_SEGMENT_BITS) | SELF_CONTINUE_BIT) as u8;
                i += 1;
                self = (self.cast_unsigned() >> 7).cast_signed();
            }
        }

    }
}

macro impl_varinttype_for_unsigned_int($signed_ty:ty => $unsigned_ty:ty) {
    unsafe impl VarIntType for $unsigned_ty {

        #[inline]
        fn decode(iter : impl Iterator<Item = u8>)
            -> Result<(Self, usize,), VarIntDecodeError>
        { <$signed_ty as VarIntType>::decode(iter).map(|(v, consumed,)|
            (v.cast_unsigned(), consumed,)
        ) }

        #[inline(always)]
        fn encode_len(self) -> usize {
            for i in (1..(size_of::<Self>() + 1)).rev() {
                let mask = Self::MAX << (7 * i);
                if ((self & mask) != 0) {
                    return i + 1;
                }
            }
            1
        }

        type EncodeBuf = <$signed_ty as VarIntType>::EncodeBuf;

        #[inline]
        fn encode(self, buf : &mut Self::EncodeBuf) -> &[u8] {
            <$signed_ty as VarIntType>::encode(self.cast_signed(), buf)
        }

    }
}

impl_varinttype_for_signed_int!(u32 => i32);
impl_varinttype_for_signed_int!(u64 => i64);
impl_varinttype_for_unsigned_int!(i32 => u32);
impl_varinttype_for_unsigned_int!(i64 => u64);


impl<T> PacketDecode for VarInt<T>
where
    T : VarIntType
{
    type Error = VarIntDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let (value, consumed,) = T::decode(buf.iter())?;
        buf.skip(consumed)?;
        Ok(VarInt(value))
    }
}

unsafe impl<T> PacketEncode for VarInt<T>
where
    T : VarIntType
{

    #[inline(always)]
    fn encode_len(&self) -> usize {
        <T as VarIntType>::encode_len(self.0)
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) {
        let mut bytes = <T as VarIntType>::EncodeBuf::default();
         unsafe { buf.write_slice(<T as VarIntType>::encode(self.0, &mut bytes)); }
    }

}


#[derive(Debug)]
pub enum VarIntDecodeError {
    Incomplete(IncompleteDecodeError),
    TooLong
}
impl From<IncompleteDecodeError> for VarIntDecodeError {
    #[inline(always)]
    fn from(err : IncompleteDecodeError) -> Self { Self::Incomplete(err) }
}
impl Display for VarIntDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Incomplete(err) => err.fmt(f),
        Self::TooLong         => write!(f, "too long")
    } }
}

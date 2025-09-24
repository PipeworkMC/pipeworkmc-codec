use crate::meta::PacketMeta;
use core::{
    ptr,
    fmt::{ self, Display, Formatter }
};


pub mod array;
mod num;
pub mod string;
pub mod vec;
#[cfg(feature = "chrono")]
pub mod datetime;


pub struct DecodeBuf<'l> {
    head : usize,
    buf  : &'l [u8]
}

impl<'l> DecodeBuf<'l> {

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.buf.get(self.head..).unwrap_or(&[])
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = u8> {
        self.as_slice().iter().cloned()
    }

    #[inline(always)]
    pub fn consumed(&self) -> usize { self.head }

}

impl<'l> DecodeBuf<'l> {

    pub fn read(&mut self) -> Result<u8, IncompleteDecodeError> {
        let b = *self.buf.get(self.head).ok_or(IncompleteDecodeError)?;
        self.head += 1;
        Ok(b)
    }

    pub fn read_vec(&mut self, count : usize) -> Result<Vec<u8>, IncompleteDecodeError> {
        let b = self.buf.get(self.head..(self.head + count)).ok_or(IncompleteDecodeError)?;
        self.head += count;
        Ok(b.to_vec())
    }

    pub fn read_remaining(&mut self) -> &[u8] {
        let b = self.buf.get(self.head..).unwrap_or_default();
        self.head = self.buf.len();
        b
    }

    #[inline(always)]
    pub fn read_arr<const COUNT : usize>(&mut self) -> Result<[u8; COUNT], IncompleteDecodeError> {
        let mut buf = [0u8; COUNT];
        self.read_buf(&mut buf)?;
        Ok(buf)
    }

    pub fn read_buf(&mut self, buf : &mut [u8]) -> Result<(), IncompleteDecodeError> {
        let b = self.buf.get(self.head..(self.head + buf.len())).ok_or(IncompleteDecodeError)?;
        self.head += buf.len();
        unsafe { ptr::copy_nonoverlapping(
            b.as_ptr(),
            buf.as_mut_ptr(),
            buf.len()
        ); }
        Ok(())
    }

    pub fn skip(&mut self, count : usize) -> Result<(), IncompleteDecodeError> {
        let next_head = self.head + count;
        if (next_head > self.buf.len()) {
            Err(IncompleteDecodeError)
        } else {
            self.head = next_head;
            Ok(())
        }
    }

}

impl<'l> From<&'l [u8]> for DecodeBuf<'l> {
    #[inline]
    fn from(buf : &'l [u8]) -> Self {
        Self { head : 0, buf }
    }
}


pub trait PacketDecode
where
    Self : Sized
{
    type Error;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>;
}


pub trait PrefixedPacketDecode
where
    Self : Sized
{
    type Error;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>;
}

impl<P> PrefixedPacketDecode for P
where
    P                                               : PacketDecode + PacketMeta,
    <P as PacketDecode>::Error                      : From<IncompleteDecodeError>,
    PrefixedDecodeError<<P as PacketDecode>::Error> : From<<P as PacketDecode>::Error>
{
    type Error = PrefixedDecodeError<<P as PacketDecode>::Error>;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let prefix = buf.read()?;
        if (prefix == <P as PacketMeta>::PREFIX) {
            Ok(<P as PacketDecode>::decode(buf)?)
        } else {
            Err(PrefixedDecodeError::UnknownPrefix {
                found    : prefix,
                expected : Some(<P as PacketMeta>::PREFIX)
            })
        }
    }
}


#[derive(Debug)]
pub struct IncompleteDecodeError;

impl Display for IncompleteDecodeError {
    #[inline(always)]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { write!(f, "missing bytes") }
}


#[derive(Debug)]
pub enum PrefixedDecodeError<E> {
    UnknownPrefix {
        found    : u8,
        expected : Option<u8>
    },
    Error(E)
}

impl<E> From<IncompleteDecodeError> for PrefixedDecodeError<E>
where
    E : From<IncompleteDecodeError>
{
    #[inline]
    fn from(err : IncompleteDecodeError) -> Self {
        Self::Error(E::from(err))
    }
}

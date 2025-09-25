//! Traits and data structures for decoding packets.


use crate::meta::PacketMeta;
use core::fmt::{ self, Display, Formatter };


pub mod array;
mod num;
pub mod string;
pub mod vec;
#[cfg(feature = "chrono")]
pub mod datetime;


/// A container for an iterator over the bytes in the packet to decode.
pub struct DecodeIter<I>
where
    I : ExactSizeIterator<Item = u8>
{ iter : I }

impl<I> DecodeIter<I>
where
    I : ExactSizeIterator<Item = u8>
{

    /// Reads a single byte from the iterator.
    ///
    /// This is similar to calling [`Iterator::next`], but returns a `Result` instead of an `Option`.
    pub fn read(&mut self) -> Result<u8, IncompleteDecodeError> {
        let b = self.iter.next().ok_or(IncompleteDecodeError)?;
        Ok(b)
    }

    /// Reads `count` bytes from the iterator into a vector.
    pub fn read_vec(&mut self, count : usize) -> Result<Vec<u8>, IncompleteDecodeError> {
        let mut buf = Vec::with_capacity(count);
        for _ in 0..count { buf.push(self.iter.next().ok_or(IncompleteDecodeError)?); }
        Ok(buf)
    }

    /// Reads `N` bytes from the iterator into an array.
    #[inline(always)]
    pub fn read_arr<const N : usize>(&mut self) -> Result<[u8; N], IncompleteDecodeError> {
        Ok(self.iter.next_chunk::<N>().map_err(|_| IncompleteDecodeError)?)
    }

    /// Reads enough bytes from the iterator to fill the buffer.
    pub fn read_buf(&mut self, buf : &mut [u8]) -> Result<(), IncompleteDecodeError> {
        for i in 0..buf.len() {
            // SAFETY: `i` is always less than `buf.len()`.
            unsafe { *buf.get_unchecked_mut(i) = self.iter.next().ok_or(IncompleteDecodeError)?; }
        }
        Ok(())
    }

    /// Skips the next `count` bytes in the iterator.
    pub fn skip(&mut self, count : usize) -> Result<(), IncompleteDecodeError> {
        for _ in 0..count { self.iter.next().ok_or(IncompleteDecodeError)?; }
        Ok(())
    }

}

impl<I> From<I> for DecodeIter<I>
where
    I : ExactSizeIterator<Item = u8>
{
    #[inline(always)]
    fn from(iter : I) -> Self { Self { iter } }
}

impl<I> Iterator for DecodeIter<I>
where
    I : ExactSizeIterator<Item = u8>
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.iter.len();
        (len, Some(len),)
    }
}
impl<I> ExactSizeIterator for DecodeIter<I>
where
    I : ExactSizeIterator<Item = u8>
{ }


/// A data structure which can be decoded from bytes.
pub trait PacketDecode
where
    Self : Sized
{
    /// The error type returned when decoding fails.
    type Error;

    /// Decode a value of this type from a byte iterator.
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>;
}


/// A data structure which can be decoded from bytes.
///
/// Unlike [`PacketDecode`], [`PrefixedPacketDecode`] should also include packet IDs in the decoding process.
pub trait PrefixedPacketDecode
where
    Self : Sized
{
    /// The error type returned when decoding fails.
    type Error;

    /// Decode a value of this type from a byte iterator.
    fn decode_prefixed<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>;
}

impl<P> PrefixedPacketDecode for P
where
    P                                               : PacketDecode + PacketMeta,
    <P as PacketDecode>::Error                      : From<IncompleteDecodeError>,
    PrefixedDecodeError<<P as PacketDecode>::Error> : From<<P as PacketDecode>::Error>
{
    type Error = PrefixedDecodeError<<P as PacketDecode>::Error>;

    fn decode_prefixed<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let prefix = iter.read()?;
        if (prefix == <P as PacketMeta>::PREFIX) {
            Ok(<P as PacketDecode>::decode(iter)?)
        } else {
            Err(PrefixedDecodeError::UnknownPrefix {
                found    : prefix,
                expected : Some(<P as PacketMeta>::PREFIX)
            })
        }
    }
}


/// The byte iterator did not provide enough data to fully decode a value.
#[derive(Debug)]
pub struct IncompleteDecodeError;

impl Display for IncompleteDecodeError {
    #[inline(always)]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { write!(f, "missing bytes") }
}


/// A packet ID is not recognised, or some other error occured.
///
/// Used by blanket implementations of [`PrefixedPacketDecode`] on types implementing [`PacketDecode`].
#[derive(Debug)]
pub enum PrefixedDecodeError<E> {

    /// The packet ID is not recognised.
    UnknownPrefix {
        /// The ID of the decoded packet.
        found    : u8,
        /// The supported packet ID.
        ///If there are multiple support packet IDs, this will be `None`.
        expected : Option<u8>
    },

    /// Some other error occured.
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

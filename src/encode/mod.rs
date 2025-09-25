//! Traits and data structures for encoding packets.


use crate::{
    meta::PacketMeta,
    varint::VarInt
};
use core::{
    mem::{ self, MaybeUninit },
    ptr
};


mod num;
mod refs;
mod option;
pub mod slice;
mod str;
mod tuple;
#[cfg(feature = "chrono")]
mod datetime;


/// A buffer of bytes that will be in the packet.
pub struct EncodeBuf {
    head : usize,
    buf  : Box<[MaybeUninit<u8>]>
}

impl EncodeBuf {

    /// Returns the current written bytes as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { mem::transmute::<&[MaybeUninit<u8>], &[u8]>(
            self.buf.get_unchecked(..self.head)
        ) }
    }

    /// Returns the current written bytes as an iterator.
    ///
    /// Using the returned iterator will not affect `self`.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = u8> {
        self.as_slice().iter().cloned()
    }

    /// Returns the number of bytes written to this buffer.
    #[inline(always)]
    pub fn written(&self) -> usize { self.head }

    /// Returns the total length of this buffer.
    #[inline(always)]
    pub fn len(&self) -> usize { self.buf.len() }

    /// Returns the inner buffer.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that this buffer has been completely filled.
    #[inline(always)]
    pub unsafe fn into_inner(self) -> Box<[u8]> {
        unsafe { self.buf.assume_init() }
    }

    /// Returns the inner buffer, converted into a vector without clones or allocations.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that this buffer has been completely filled.
    #[inline]
    pub unsafe fn into_inner_as_vec(self) -> Vec<u8> {
        unsafe { self.buf.assume_init() }.into_vec()
    }

}

impl EncodeBuf {

    /// Creates a new empty [`EncodeBuf`] with enough space allocated to write `len` bytes.
    ///
    /// [`PacketEncode::encode_len`] and [`PrefixedPacketEncode::encode_len`] can be used to calculate the length of a packet in advance.
    #[inline]
    pub fn new(len : usize) -> Self {
        Self { head : 0, buf : Box::new_uninit_slice(len) }
    }

    /// Creates a new empty [`EncodeBuf`] with enough space allocated to write `len` more bytes.
    /// The size will be written at the start of the packet as a [`VarInt::<u32>`](VarInt).
    ///
    /// [`PacketEncode::encode_len`] and [`PrefixedPacketEncode::encode_len`] can be used to calculate the length of a packet in advance.
    #[inline]
    pub fn new_len_prefixed(len : usize) -> Self {
        let len_varint = VarInt::<u32>(len as u32);
        let mut buf = Self::new(len_varint.encode_len() + len);
        unsafe { len_varint.encode(&mut buf); }
        buf
    }

    /// Writes a byte to this buffer.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that this buffer has enough space to write this byte.
    /// Writing more than `self.len()` total bytes is [*undefined behaviour*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).
    pub unsafe fn write(&mut self, b : u8) {
        unsafe { self.buf.get_unchecked_mut(self.head) }.write(b);
        self.head += 1;
    }

    /// Writes a slice of byte to this buffer.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that this buffer has enough space to write these bytes.
    /// Writing more than `self.len()` total bytes is [*undefined behaviour*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).
    pub unsafe fn write_slice(&mut self, slice : &[u8]) {
        unsafe { ptr::copy_nonoverlapping(
            slice.as_ptr(),
            mem::transmute::<&mut [MaybeUninit<u8>], &mut [u8]>(
                self.buf.get_unchecked_mut(self.head..)
            ).as_mut_ptr(),
            slice.len()
        ); }
        self.head += slice.len();
    }

}


/// A data structure which can be encoded into bytes.
///
/// ### Safety
/// The implementor is responsible for ensuring that `encode_len` returns the exact number of bytes that `encode` will write.
/// Returning an incorrect value is [*undefined behaviour*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html), as it will cause `encode` to write too few or too many bytes to the buffer.
pub unsafe trait PacketEncode {

    /// Returns the exact number of bytes that `self.encode()` will write.
    fn encode_len(&self) -> usize;

    /// Encode this value into a byte buffer.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that the given buffer has exactly `self.encode_len()` total bytes of space.
    /// Passing an incorrectly sized buffer is [*undefined behaviour*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html), as too few or too many bytes will be written to the buffer.
    unsafe fn encode(&self, buf : &mut EncodeBuf);

}


/// A data structure which can be encoded into bytes.
///
/// Unlike [`PacketEncode`], [`PrefixedPacketEncode`] should also include packet IDs in the encoding process.
///
/// ### Safety
/// The implementor is responsible for ensuring that `encode_prefixed_len` returns the exact number of bytes that `encode_prefixed` will write.
/// Returning an incorrect value is [*undefined behaviour*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html), as it will cause `encode_prefixed` to write too few or too many bytes to the buffer.
pub unsafe trait PrefixedPacketEncode {

    /// Returns the exact number of bytes that `self.encode_prefixed()` will write.
    fn encode_prefixed_len(&self) -> usize;

    /// Encode this value into a byte buffer.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that the given buffer has exactly `self.encode_prefixed_len()` total bytes of space.
    /// Passing an incorrectly sized buffer is [*undefined behaviour*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html), as too few or too many bytes will be written to the buffer.
    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf);

}

unsafe impl<P> PrefixedPacketEncode for P
where
    P : PacketEncode + PacketMeta
{

    #[inline(always)]
    fn encode_prefixed_len(&self) -> usize {
        1 + <P as PacketEncode>::encode_len(self)
    }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe {
        buf.write(<P as PacketMeta>::PREFIX);
        <P as PacketEncode>::encode(self, buf);
    } }

}

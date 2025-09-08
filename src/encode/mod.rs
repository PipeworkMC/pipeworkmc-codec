use crate::meta::PacketMeta;
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


pub struct EncodeBuf {
    head : usize,
    buf  : Box<[MaybeUninit<u8>]>
}

impl EncodeBuf {

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { mem::transmute::<&[MaybeUninit<u8>], &[u8]>(
            self.buf.get_unchecked(..self.head)
        ) }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = u8> {
        self.as_slice().iter().cloned()
    }

    #[inline(always)]
    pub fn written(&self) -> usize { self.head }

}

impl EncodeBuf {

    #[inline]
    pub fn new(len : usize) -> Self {
        Self { head : 0, buf : Box::new_uninit_slice(len) }
    }

    pub unsafe fn write(&mut self, b : u8) {
        unsafe { self.buf.get_unchecked_mut(self.head) }.write(b);
        self.head += 1;
    }

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


pub unsafe trait PacketEncode {

    fn encode_len(&self) -> usize;

    unsafe fn encode(&self, buf : &mut EncodeBuf);

}


pub unsafe trait PrefixedPacketEncode {

    fn encode_prefixed_len(&self) -> usize;

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

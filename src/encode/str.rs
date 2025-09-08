use crate::encode::{
    PacketEncode,
    EncodeBuf
};
use crate::varint::VarInt;
use std::borrow::Cow;


unsafe impl PacketEncode for str {

    #[inline]
    fn encode_len(&self) -> usize {
        VarInt::<u32>(self.len() as u32).encode_len()
        + self.len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        VarInt::<u32>(self.len() as u32).encode(buf);
        buf.write_slice(self.as_bytes());
    } }

}


unsafe impl PacketEncode for Cow<'_, str> {

    #[inline(always)]
    fn encode_len(&self) -> usize { str::encode_len(self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        str::encode(self, buf)
    } }

}


unsafe impl PacketEncode for String {

    #[inline(always)]
    fn encode_len(&self) -> usize { str::encode_len(self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        str::encode(self, buf)
    } }

}

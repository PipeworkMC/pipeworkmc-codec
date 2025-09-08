use crate::encode::{
    PacketEncode,
    EncodeBuf
};


unsafe impl<T> PacketEncode for Option<T>
where
    T : PacketEncode
{

    #[inline]
    fn encode_len(&self) -> usize { match (self) {
        Some(inner) => true.encode_len() + inner.encode_len(),
        None        => false.encode_len()
    } }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Some(inner) => {
            true.encode(buf);
            inner.encode(buf);
        },
        None => { false.encode(buf); }
    } } }

}

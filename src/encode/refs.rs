use crate::encode::{
    PacketEncode,
    EncodeBuf
};


unsafe impl<T> PacketEncode for &T
where
    T : PacketEncode + ?Sized
{

    #[inline(always)]
    fn encode_len(&self) -> usize { T::encode_len(*self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        T::encode(*self, buf)
    } }

}


unsafe impl<T> PacketEncode for &mut T
where
    T : PacketEncode + ?Sized
{

    #[inline(always)]
    fn encode_len(&self) -> usize { T::encode_len(*self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        T::encode(*self, buf)
    } }

}

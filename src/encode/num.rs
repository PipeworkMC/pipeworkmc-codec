use crate::encode::{
    PacketEncode,
    EncodeBuf
};
use uuid::Uuid;


macro impl_packetencode_for_num($ty:ty) {
    unsafe impl PacketEncode for $ty {

        #[inline(always)]
        fn encode_len(&self) -> usize { size_of::<Self>() }

        #[inline]
        unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
            buf.write_slice(&self.to_be_bytes())
        } }

    }
}

impl_packetencode_for_num!(u8);
impl_packetencode_for_num!(i8);
impl_packetencode_for_num!(u16);
impl_packetencode_for_num!(i16);
impl_packetencode_for_num!(u32);
impl_packetencode_for_num!(i32);
impl_packetencode_for_num!(u64);
impl_packetencode_for_num!(i64);
impl_packetencode_for_num!(u128);
impl_packetencode_for_num!(i128);
impl_packetencode_for_num!(f32);
impl_packetencode_for_num!(f64);


unsafe impl PacketEncode for bool {

    #[inline(always)]
    fn encode_len(&self) -> usize { size_of::<u8>() }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        buf.write(if (*self) { 1u8 } else { 0u8 })
    } }

}


unsafe impl PacketEncode for Uuid {

    #[inline(always)]
    fn encode_len(&self) -> usize { size_of::<u128>() }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.as_u128().encode(buf);
    } }

}

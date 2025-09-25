use crate::decode::{
    PacketDecode,
    DecodeIter,
    IncompleteDecodeError
};
use uuid::Uuid;


macro impl_packetdecode_for_num($ty:ty) {
    impl PacketDecode for $ty {
        type Error = IncompleteDecodeError;

        fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
        where
            I : ExactSizeIterator<Item = u8>
        { Ok(Self::from_be_bytes(iter.read_arr()?)) }
    }
}

impl_packetdecode_for_num!(u8);
impl_packetdecode_for_num!(i8);
impl_packetdecode_for_num!(u16);
impl_packetdecode_for_num!(i16);
impl_packetdecode_for_num!(u32);
impl_packetdecode_for_num!(i32);
impl_packetdecode_for_num!(u64);
impl_packetdecode_for_num!(i64);
impl_packetdecode_for_num!(u128);
impl_packetdecode_for_num!(i128);
impl_packetdecode_for_num!(f32);
impl_packetdecode_for_num!(f64);


impl PacketDecode for bool {
    type Error = IncompleteDecodeError;

    #[inline(always)]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(iter.read()? != 0) }
}

impl PacketDecode for Uuid {
    type Error = IncompleteDecodeError;

    #[inline(always)]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Uuid::from_u128(<_>::decode(iter)?)) }
}

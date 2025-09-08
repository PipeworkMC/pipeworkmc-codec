use crate::decode::{
    PacketDecode,
    DecodeBuf
};
use crate::varint::{
    VarInt,
    VarIntDecodeError
};
use core::mem::MaybeUninit;


impl<const N : usize, T> PacketDecode for [T; N]
where
    T : PacketDecode
{
    type Error = ArrayDecodeError<T::Error>;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let length = *VarInt::<u32>::decode(buf).map_err(ArrayDecodeError::Length)? as usize;
        if (length != N) {
            return Err(ArrayDecodeError::BadLength { len : length, expected : N });
        }
        let mut arr = [const { MaybeUninit::uninit() }; N];
        for i in 0..length {
            match (T::decode(buf).map_err(|err| ArrayDecodeError::Entry { index : i, err })) {
                Ok(item) => unsafe { arr.get_unchecked_mut(i).write(item); },
                Err(err) => {
                    for j in 0..i {
                        unsafe { arr.get_unchecked_mut(j).assume_init_drop(); }
                    }
                    return Err(err);
                }
            }
        }
        Ok(unsafe { MaybeUninit::array_assume_init(arr) })
    }
}


#[derive(Debug)]
pub enum ArrayDecodeError<E> {
    Length(VarIntDecodeError),
    BadLength {
        len      : usize,
        expected : usize
    },
    Entry {
        index : usize,
        err   : E
    }
}

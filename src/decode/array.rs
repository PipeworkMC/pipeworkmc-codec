//! `[T; N]` decoder.


use crate::decode::{
    PacketDecode,
    DecodeIter
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

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let length = *VarInt::<u32>::decode(iter).map_err(ArrayDecodeError::Length)? as usize;
        if (length != N) {
            return Err(ArrayDecodeError::BadLength { len : length, expected : N });
        }
        let mut arr = [const { MaybeUninit::uninit() }; N];
        for i in 0..N {
            match (T::decode(iter).map_err(|err| ArrayDecodeError::Item { index : i, err })) {
                // SAFETY: `i` is guaranteed to be less than `arr.len()`.
                Ok(item) => unsafe { arr.get_unchecked_mut(i).write(item); },
                Err(err) => {
                    for j in 0..i {
                        // SAFETY: Up to, but not including, `i` items in `arr` are guaranteed to have been written.
                        unsafe { arr.get_unchecked_mut(j).assume_init_drop(); }
                    }
                    return Err(err);
                }
            }
        }
        // SAFETY: All bytes in `arr` were written.
        Ok(unsafe { MaybeUninit::array_assume_init(arr) })
    }
}


/// Returned by packet decoders when a `[T; N]` was not decoded successfully.
#[derive(Debug)]
pub enum ArrayDecodeError<E> {
    /// The length of the array failed to decode.
    Length(VarIntDecodeError),
    /// The length of the decoded array does not match the expected length.
    BadLength {
        /// The length of the decoded array.
        len      : usize,
        /// The expected length.
        expected : usize
    },
    /// An item in the array could not be decoded.
    Item {
        /// The index of the item that was not decoded.
        index : usize,
        /// The error.
        err   : E
    }
}

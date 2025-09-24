use crate::encode::{
    PacketEncode,
    EncodeBuf
};
use chrono::{ DateTime, Utc };


unsafe impl PacketEncode for DateTime<Utc> {
    fn encode_len(&self) -> usize {
        self.timestamp().encode_len()
        + self.timestamp_subsec_nanos().encode_len()
    }
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.timestamp().encode(buf);
        self.timestamp_subsec_nanos().encode(buf);
    } }
}

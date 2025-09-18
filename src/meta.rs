use atomic_enum::atomic_enum;


pub trait PacketMeta {
    const STATE  : PacketState;
    const BOUND  : PacketBound;
    const PREFIX : u8;
    const KICK   : bool        = false;
}

#[derive(PartialEq, Eq, Hash)]
#[atomic_enum]
pub enum PacketState {
    Handshake,
    Status,
    Login,
    Config,
    Play
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PacketBound {
    S2C,
    C2S
}

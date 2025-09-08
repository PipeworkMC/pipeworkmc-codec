use atomic_enum::atomic_enum;


pub trait PacketMeta {
    const STATE  : PacketState;
    const BOUND  : PacketBound;
    const PREFIX : u8;
}

#[atomic_enum]
#[derive(PartialEq, Eq, Hash, Default)]
pub enum PacketState {
    #[default]
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

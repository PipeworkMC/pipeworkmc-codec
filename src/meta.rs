//! Packet metadata.


use core::{
    mem::transmute,
    sync::atomic::{
        AtomicU8,
        Ordering as AtomicOrdering
    }
};


/// Packet metadata.
pub trait PacketMeta {
    /// The state in which this packet will be sent.
    const STATE  : PacketState;
    /// The direction that this packet will be sent.
    const BOUND  : PacketBound;
    /// This ID of this packet.
    const PREFIX : u8;
    /// Whether this packet will kick the player from the server.
    const KICK   : bool        = false;
}


/// The state in which a packet will be sent.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum PacketState {

    /// Handshaking
    Handshake,

    /// Status
    Status,

    /// Login
    Login,

    /// Configuration
    Config,

    /// Play
    Play

}


/// A `PacketState` which can be safely shared between threads.
///
/// This type has the same size, alignment, and bit validity as a [`u8`].
#[derive(Debug)]
#[repr(transparent)]
pub struct AtomicPacketState(AtomicU8);
impl AtomicPacketState {

    /// Creates a new atomic packet state.
    #[inline(always)]
    pub const fn new(state : PacketState) -> Self {
        Self(AtomicU8::new(state as u8))
    }

    /// Consumes the atomic and reutrns the contained value.
    #[inline(always)]
    pub const fn into_inner(self) -> PacketState {
        // SAFETY: This is safe because the inner value of the `AtomicU8` will always be a valid `PacketState` ordinal.
        unsafe { transmute::<u8, PacketState>(self.0.into_inner()) }
    }

    /// Loads a value from the atomic.
    ///
    /// ### Panics
    /// Panics if `order` is [`Release`](AtomicOrdering::Release) or [`AcqRel`](AtomicOrdering::AcqRel).
    #[inline(always)]
    pub fn load(&self, order : AtomicOrdering) -> PacketState {
        // SAFETY: This is safe because the inner value of the `AtomicU8` will always be a valid `PacketState` ordinal.
        unsafe { transmute::<u8, PacketState>(self.0.load(order)) }
    }

    /// Stores a value into the atomic.
    ///
    /// ### Panics
    /// Panics if `order` is [`Acquire`](AtomicOrdering::Acquire) or [`AcqRel`](AtomicOrdering::AcqRel).
    #[inline(always)]
    pub fn store(&self, val : PacketState, order : AtomicOrdering) {
        self.0.store(val as u8, order)
    }

    /// Stores a value into the atomic, returning the previous value.
    #[inline(always)]
    pub fn swap(&self, val : PacketState, order : AtomicOrdering) -> PacketState {
        // SAFETY: This is safe because the inner value of the `AtomicU8` will always be a valid `PacketState` ordinal.
        unsafe { transmute::<u8, PacketState>(self.0.swap(val as u8, order)) }
    }

    /// Stores a value into the atomic if the current value is the same as the `current` value.
    ///
    /// The return value is a result indicating whether the new value was written and containing the previous value.
    /// On success this value is guaranteed to be equal to `current`.
    #[inline(always)]
    pub fn compare_exchange(&self, current : PacketState, new : PacketState, success : AtomicOrdering, failure : AtomicOrdering) -> Result<PacketState, PacketState> {
        self.0.compare_exchange(current as u8, new as u8, success, failure)
            // SAFETY: This is safe because the inner value of the `AtomicU8` will always be a valid `PacketState` ordinal.
            .map     (|v| unsafe { transmute::<u8, PacketState>(v) })
            .map_err (|v| unsafe { transmute::<u8, PacketState>(v) })
    }

    /// Stores a value into the atomic if the current value is the same as the `current` value.
    ///
    /// Unlike [`compare_exchange`], this function is allowed to spuriously fail even when the comparison succeeds, which can result in more efficient code on some platforms.
    /// The return value is a result indicaing whether the new value was written and containing the previous value.
    #[inline(always)]
    pub fn compare_exchange_weak(&self, current : PacketState, new : PacketState, success : AtomicOrdering, failure : AtomicOrdering) -> Result<PacketState, PacketState> {
        self.0.compare_exchange(current as u8, new as u8, success, failure)
            // SAFETY: This is safe because the inner value of the `AtomicU8` will always be a valid `PacketState` ordinal.
            .map     (|v| unsafe { transmute::<u8, PacketState>(v) })
            .map_err (|v| unsafe { transmute::<u8, PacketState>(v) })
    }

    /// Fetches the value, and applies a function to it that returns an optional new value.
    /// Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else `Err(previous_value)`.
    ///
    /// Note: This may call the function multiple times if the value has been changed from other threads in the meantime,
    ///  as long as the function `Some(_)`, but the function will have been applied only once to the stored value.
    #[inline(always)]
    pub fn fetch_update<F>(&self, set_order : AtomicOrdering, fetch_order : AtomicOrdering, mut f : F) -> Result<PacketState, PacketState>
    where
        F : FnMut(PacketState) -> Option<PacketState>
    {
        // SAFETY: This is safe because the inner value of the `AtomicU8` will always be a valid `PacketState` ordinal.
        self.0.fetch_update(set_order, fetch_order, |v| f(unsafe { transmute::<u8, PacketState>(v) }).map(|s| s as u8))
            .map     (|v| unsafe { transmute::<u8, PacketState>(v) })
            .map_err (|v| unsafe { transmute::<u8, PacketState>(v) })
    }

}


/// The direction that a packet will be sent.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PacketBound {
    /// Server to client (clientbound).
    S2C,
    /// Client to server (serverbound).
    C2S
}

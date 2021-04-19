use crate::Magic;

/// Represents an error that occured during an operation
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Some field would overflow (range `0 - 1023` for 32 bits, `0 - 65535` for 64 bits)
    Overflow,

    /// Could not unpack, field contained unknown api version
    /// (only [`Magic::V0`] supported for now)
    UnknownMagic(u64),

    /// Could not unpack, field contained an unsupported api version
    UnsupportedMagic(Magic),
}

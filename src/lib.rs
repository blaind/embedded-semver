//! Compact representation of semantic versions as an i32, u32 i64 or u64
//!
//! Limitations:
//! * Storage format is not standards-conformant (none exists at the time of writing)
//! * 32-bit values can represent values in range of `major/minor/patch` = `0 - 1023` (10 bits)
//! * 64-bit values can represent values in range of `major/minor/patch` = `0 - 65535` (16 bits)
//! * Other than major/minor/patch features (e.g pre-release) are not supported
//!
//! # Examples
//!
//! Encode a semver to an integer
//! ```rust
//! use embedded_semver::Semver;
//!
//! let version = Semver::new(1, 0, 20);
//!
//! let int_semver = version.to_i32().unwrap();
//! assert_eq!(int_semver, 83886081);
//! assert_eq!(&int_semver.to_le_bytes(), &[0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0101]);
//! ```
//!
//! Decode an integer into a semver:
//! ```rust
//! # use embedded_semver::Semver;
//! #
//! let int_semver: i32 = 83886081;
//!
//! let version = Semver::from_i32(int_semver).unwrap();
//! assert_eq!(version, Semver::new(1, 0, 20));
//! ```
//!
//! # Binary format
//!
//! * Fields are packed with the most significant bit first (see [Msb0](https://docs.rs/bitvec/0.22.3/bitvec/order/struct.Msb0.html))
//! * Format contains a 2-bit (on 32-bit values) and 4-bit (on 64-bit values) api version, which
//!   allows for extensibility and/or api changes in future
//!
//! ## 32 bits
//! The binary format for `i32` and `u32` is represented below. Fields:
//!
//! * `version`: First two bits represent the API version (packed data format). Currenly 0 = V0
//! * `major`: Next 10 bits represent major in range of 0-1023
//! * `minor`: Next 10 bits represent minor in range of 0-1023
//! * `patch`: Next 10 bits represent patch in range of 0-1023
//!
//! ```text
//! 0         2        12        22        32
//! ├────┴────┼┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┤
//! │ API ver │ Major   │ Minor   │ Patch   │
//! │ u2      │ u10     │ u10     │ u10     │
//! └─────────┴─────────┴─────────┴─────────┘
//! ```
//!
//! ## 64 bits
//! The binary format for `i64` and `u64` is represented below. Fields:
//!
//! * `version`: First four bits represent the API version (packed data format). Currenly 0 = V0
//! * `major`: Next 16 bits represent major in range of 0-65535
//! * `minor`: Next 16 bits represent minor in range of 0-65535
//! * `patch`: Next 16 bits represent patch in range of 0-65535
//! * 12 remaining bits are unused
//!
//! ```text
//! 0         4              20                36                52          64
//! ├─┴──┴─┴──┼┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┴┴┤
//! │ API ver │ Major         │ Minor           │ Patch           | (unused)  │
//! │ u4      │ u16           │ u16             │ u16             | u12       │
//! └─────────┴───────────────┴─────────────────┴─────────────────┴───────────┘
//! ```
//!
//! # Notes about conversions
//!
//! Note that the binary representations in i32 and i64 are not the same:
//! ```rust
//! # use embedded_semver::{Semver, Error, Magic};
//! #
//! let value: i32 = 16904511;
//!
//! assert_eq!(Semver::from_i32(value).unwrap(), Semver::new(1023, 1, 5));
//! assert_eq!(
//!     Semver::from_i64(value as i64).unwrap_err(),
//!     Error::UnsupportedMagic(Magic::V3)
//! );
//! ```
//!
//! However, the underlying bits in u32 and i32 represent the same values:
//! ```rust
//! # use embedded_semver::{Semver, Error};
//! #
//! let value: i32 = 16843009;
//! let bytes = value.to_le_bytes();
//!
//! let i32_value = i32::from_le_bytes(bytes.clone());
//! let u32_value = u32::from_le_bytes(bytes);
//!
//! assert_eq!(Semver::from_i32(i32_value).unwrap(), Semver::new(1, 1, 5));
//! assert_eq!(Semver::from_u32(u32_value).unwrap(), Semver::new(1, 1, 5));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub mod prelude {
    pub use crate::{Error, Magic, Semver};
}

mod error;
mod helpers;
mod sizes;
mod version;

pub use error::Error;
pub use version::Semver;

/// Magic number - storage format
///
/// Currently all operations are based on [`Magic::V0`], others are left
/// for future extensibility.
#[derive(Debug, PartialEq)]
pub enum Magic {
    // can have max 4 (2 bits)
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
}

impl Default for Magic {
    fn default() -> Self {
        Magic::V0
    }
}

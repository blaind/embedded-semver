# embedded-semver &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Version]][docs] [![Lines of Code]][github]

[Build Status]: https://img.shields.io/github/workflow/status/blaind/embedded-semver/test
[actions]: https://github.com/blaind/embedded-semver/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/embedded-semver.svg
[crates.io]: https://crates.io/crates/embedded-semver
[Lines of Code]: https://tokei.rs/b1/github/blaind/embedded-semver?category=code
[github]: https://github.com/blaind/embedded-semver

[Docs Version]: https://docs.rs/embedded-semver/badge.svg
[docs]: https://docs.rs/embedded-semver

Compact representation of semantic versions as an i32, u32 i64 or u64

Because the integer formats have limited space, there are limits for semver field
(major, minor, patch) sizes depending on type (`0 - 1023` for 32 bits, `0 - 65535` for 64 bits).

## Examples

### Encoding

Encode a semver to an integer

```rust
use embedded_semver::Semver;

let version = Semver::new(1, 0, 20);
let int_semver = version.to_i32().unwrap();

assert_eq!(int_semver, 83886081);
assert_eq!(&int_semver.to_le_bytes(), &[
  0b0000_0001,
  0b0000_0000,
  0b0000_0000,
  0b0000_0101
]);
```

### Decoding

Decode an integer into a semver:

```rust
use embedded_semver::Semver;

let int_semver: i32 = 83886081;

let version = Semver::from_i32(int_semver).unwrap();
assert_eq!(version, Semver::new(1, 0, 20));
```

## Feature flags

For no-std support, add as a dependency without default features:

```text
embedded-semver = { version = "...", default-features = false }
```

## Binary format

* Fields are packed with the most significant bit first (see [Msb0](https://docs.rs/bitvec/0.22.3/bitvec/order/struct.Msb0.html))
* Format contains a 2-bit (on 32-bit values) and 4-bit (on 64-bit values) api version, which
  allows for extensibility and/or api changes in future

### 32 bits

The binary format for `i32` and `u32` is represented below. Fields:

* `version`: First two bits represent the API version (packed data format). Currenly 0 = V0
* `major`: Next 10 bits represent major in range of 0-1023
* `minor`: Next 10 bits represent minor in range of 0-1023
* `patch`: Next 10 bits represent patch in range of 0-1023

```text
0         2        12        22        32
├────┴────┼┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┤
│ API ver │ Major   │ Minor   │ Patch   │
│ u2      │ u10     │ u10     │ u10     │
└─────────┴─────────┴─────────┴─────────┘
```

### 64 bits

The binary format for `i64` and `u64` is represented below. Fields:

* `version`: First four bits represent the API version (packed data format). Currenly 0 = V0
* `major`: Next 16 bits represent major in range of 0-65535
* `minor`: Next 16 bits represent minor in range of 0-65535
* `patch`: Next 16 bits represent patch in range of 0-65535
* 12 remaining bits are unused

```text
0         4              20                36                52          64
├─┴──┴─┴──┼┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┴┼┴┴┴┴┴┴┴┴┴┴┴┤
│ API ver │ Major         │ Minor           │ Patch           | (unused)  │
│ u4      │ u16           │ u16             │ u16             | u12       │
└─────────┴───────────────┴─────────────────┴─────────────────┴───────────┘
```

## Notes about conversions

Note that the binary representations in i32 and i64 are not the same:

```rust
use embedded_semver::{Semver, Error, Magic};

let value: i32 = 16904511;
assert_eq!(Semver::from_i32(value).unwrap(), Semver::new(1023, 1, 5));
assert_eq!(
    Semver::from_i64(value as i64).unwrap_err(),
    Error::UnsupportedMagic(Magic::V3)
);
```

However, the underlying bits in u32 and i32 represent the same values:

```rust
use embedded_semver::{Semver, Error};

let value: i32 = 16843009;
let bytes = value.to_le_bytes();
let i32_value = i32::from_le_bytes(bytes.clone());
let u32_value = u32::from_le_bytes(bytes);
assert_eq!(Semver::from_i32(i32_value).unwrap(), Semver::new(1, 1, 5));
assert_eq!(Semver::from_u32(u32_value).unwrap(), Semver::new(1, 1, 5));
```

## Performance

No need to worry :)

```text
test bench_from_i32 ... bench:          51 ns/iter (+/- 1)
test bench_from_i64 ... bench:          54 ns/iter (+/- 1)
test bench_to_i32   ... bench:          75 ns/iter (+/- 1)
test bench_to_i64   ... bench:          84 ns/iter (+/- 2)
```

## Future plans

None currently

## License
Compact semantic version representation
* <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or
* <a href="LICENSE-MIT">MIT license</a>

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the software by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

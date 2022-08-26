use bitvec::prelude::*;

use crate::{helpers::num_as_bv, sizes, Error, Magic};

/// Represents a version number conforming to the semantic versioning scheme
///
/// Conversions:
/// * To an integer: [`Semver::to_i32`], [`Semver::to_u32`], [`Semver::to_i64`]
///   and [`Semver::to_u64`]
/// * From an integer: [`Semver::from_i32`], [`Semver::from_u32`], [`Semver::from_i64`]
///   and [`Semver::from_u64`]
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Software_versioning#Degree_of_compatibility)
/// explains semantic versioning and the fields in detail.
#[derive(Debug, PartialEq)]
pub struct Semver {
    /// The semantic versioning major version (high risk)
    pub major: usize,
    /// The semantic versioning minor version (medium risk)
    pub minor: usize,
    /// The semantic versioning patch version (lowest risk)
    pub patch: usize,

    /// Magic with what the Semver has been or will be packed
    magic: Magic,
}

impl Semver {
    /// Helper for constructing new instance
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
            magic: Default::default(),
        }
    }

    /// Construct from an i64
    pub fn from_i64(n: i64) -> Result<Self, Error> {
        let bytes = n.to_le_bytes();
        let bv = bytes.view_bits::<Msb0>();
        let sizes = sizes::size_iterator(&sizes::I64_SIZES);
        Ok(Self::from_size_iterator(&bv, sizes)?)
    }

    /// Construct from an u64
    pub fn from_u64(n: u64) -> Result<Self, Error> {
        let i64 = i64::from_le_bytes(n.to_le_bytes());
        Self::from_i64(i64)
    }

    /// Construct from an i32
    pub fn from_i32(n: i32) -> Result<Self, Error> {
        let bytes = n.to_le_bytes();
        let bv = bytes.view_bits::<Msb0>();
        let sizes = sizes::size_iterator(&sizes::I32_SIZES);
        Ok(Self::from_size_iterator(bv, sizes)?)
    }

    /// Construct from an u32
    pub fn from_u32(n: u32) -> Result<Self, Error> {
        let i32 = i32::from_le_bytes(n.to_le_bytes());
        Self::from_i32(i32)
    }

    /// Convert to an i32. Errs if any of the fields overflow
    pub fn to_i32(&self) -> Result<i32, Error> {
        let mut bv: BitArray<[u8; 4], Msb0> = BitArray::ZERO;
        let sizes = sizes::size_iterator(&sizes::I32_SIZES);
        self.append_with_size_iterator(&mut bv, sizes)?;
        Ok(i32::from_le_bytes(bv.data))
    }

    /// Convert to an u32. Errs if any of the fields overflow
    pub fn to_u32(&self) -> Result<u32, Error> {
        let val = self.to_i32()?;
        Ok(u32::from_le_bytes(val.to_le_bytes()))
    }

    /// Convert to an u64. Errs if any of the fields overflow
    pub fn to_i64(&self) -> Result<i64, Error> {
        let mut bv: BitArray<[u8; 8], Msb0> = BitArray::ZERO;
        let sizes = sizes::size_iterator(&sizes::I64_SIZES);
        self.append_with_size_iterator(&mut bv, sizes)?;
        Ok(i64::from_le_bytes(bv.data))
    }

    /// Convert to an u64. Errs if any of the fields overflow
    pub fn to_u64(&self) -> Result<u64, Error> {
        let val = self.to_i64()?;
        Ok(u64::from_le_bytes(val.to_le_bytes()))
    }

    fn from_size_iterator<const SIZE: usize>(
        bv: &BitSlice<u8, Msb0>,
        mut sizes: sizes::SizeIterator<SIZE>,
    ) -> Result<Self, Error> {
        Ok(Self {
            magic: convert_api_version(bv[sizes.next().unwrap()].load::<u64>())?,
            major: bv[sizes.next().unwrap()].load::<usize>(),
            minor: bv[sizes.next().unwrap()].load::<usize>(),
            patch: bv[sizes.next().unwrap()].load::<usize>(),
        })
    }

    fn append_with_size_iterator<const SIZE: usize, const ITER_SIZE: usize>(
        &self,
        bv: &mut BitArray<[u8; SIZE], Msb0>,
        mut sizes: sizes::SizeIterator<ITER_SIZE>,
    ) -> Result<(), Error> {
        num_as_bv(bv, &mut sizes, Magic::default() as u64)?; // for now, can be extended in future
        num_as_bv(bv, &mut sizes, self.major as u64)?;
        num_as_bv(bv, &mut sizes, self.minor as u64)?;
        num_as_bv(bv, &mut sizes, self.patch as u64)?;
        Ok(())
    }
}

fn convert_api_version(n: u64) -> Result<Magic, Error> {
    let api_version = match n {
        0 => Magic::V0,
        1 => Magic::V1,
        2 => Magic::V2,
        3 => Magic::V3,
        _ => return Err(Error::UnknownMagic(n)),
    };

    match api_version {
        Magic::V0 => (),
        _ => return Err(Error::UnsupportedMagic(api_version)),
    }

    Ok(api_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrips() {
        assert_roundtrip_i32(Semver::new(254, 500, 498));
        assert_roundtrip_i32(Semver::new(0, 0, 0));

        assert_roundtrip_u32(Semver::new(254, 500, 498));
        assert_roundtrip_u32(Semver::new(0, 0, 0));

        assert_roundtrip_i64(Semver::new(65343, 64000, 65310));
        assert_roundtrip_i64(Semver::new(0, 0, 0));

        assert_roundtrip_u64(Semver::new(65343, 64000, 65310));
        assert_roundtrip_u64(Semver::new(0, 0, 0));

        let val = Semver::new(5, 230, 150).to_i32().unwrap() as u32;
        let version = Semver::from_u32(val).unwrap();
        assert_eq!(version, Semver::new(5, 230, 150));
    }

    #[test]
    fn test_unsupported_api_version() {
        let mut bv: BitArray<[u8; 4], Msb0> = BitArray::ZERO;
        let mut iter = sizes::size_iterator(&sizes::I32_SIZES);
        num_as_bv(&mut bv, &mut iter, Magic::V2 as u64).unwrap();
        let val = i32::from_le_bytes(bv.data);

        assert_eq!(
            Semver::from_i32(val).unwrap_err(),
            Error::UnsupportedMagic(Magic::V2)
        );
    }

    #[test]
    fn test_unknown_api_version() {
        let mut bv: BitArray<[u8; 8], Msb0> = BitArray::ZERO;
        let mut iter = sizes::size_iterator(&sizes::I64_SIZES);
        num_as_bv(&mut bv, &mut iter, 13).unwrap();
        let val = i64::from_le_bytes(bv.data);

        assert_eq!(Semver::from_i64(val).unwrap_err(), Error::UnknownMagic(13));
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(Semver::from_i32(16843009).unwrap(), test_version())
    }

    #[test]
    fn test_from_u32() {
        let u32 = u32::from_le_bytes(16843009i32.to_le_bytes());
        assert_eq!(Semver::from_u32(u32).unwrap(), test_version())
    }

    #[test]
    fn test_from_i64() {
        assert_eq!(Semver::from_i64(21474902017).unwrap(), test_version())
    }

    #[test]
    fn test_from_u64() {
        let u64 = u64::from_le_bytes(21474902017i64.to_le_bytes());
        assert_eq!(Semver::from_u64(u64).unwrap(), test_version())
    }

    #[test]
    fn test_to_i32() {
        let val = test_version().to_i32().unwrap();
        assert_eq!(val, 16843009);
        assert_eq!(&val.to_le_bytes()[..], &[0b1, 0b1, 0b1, 0b1]);
    }

    #[test]
    fn test_to_i64() {
        let val = test_version().to_i64().unwrap();
        assert_eq!(
            &val.to_le_bytes()[..],
            &[0b1, 0b0, 0b1, 0b0, 0b101, 0b0, 0b0, 0b0]
        );

        assert_eq!(val, 21474902017);
    }

    #[test]
    fn test_overflow_i32() {
        assert!(Semver::new(2usize.pow(10), 2usize.pow(10), 2usize.pow(10))
            .to_i32()
            .is_ok());

        // for overflows, see sizes::I32_SIZES
        assert_eq!(
            Semver::new(2usize.pow(10) + 1, 0, 0).to_i32().unwrap_err(),
            Error::Overflow
        );

        assert_eq!(
            Semver::new(0, 2usize.pow(10) + 1, 0).to_i32().unwrap_err(),
            Error::Overflow
        );

        assert_eq!(
            Semver::new(0, 0, 2usize.pow(10) + 1).to_i32().unwrap_err(),
            Error::Overflow
        );
    }

    #[test]
    fn test_overflow_i64() {
        assert!(Semver::new(2usize.pow(16), 2usize.pow(16), 2usize.pow(16))
            .to_i64()
            .is_ok());

        // see sizes::I64_SIZES
        let overflow = 2usize.pow(16) + 1;

        assert_eq!(
            Semver::new(overflow, 0, 0).to_i64().unwrap_err(),
            Error::Overflow
        );

        assert_eq!(
            Semver::new(0, overflow, 0).to_i64().unwrap_err(),
            Error::Overflow
        );

        assert_eq!(
            Semver::new(0, 0, overflow).to_i64().unwrap_err(),
            Error::Overflow
        );
    }

    fn test_version() -> Semver {
        Semver {
            major: 1,
            minor: 1,
            patch: 5,
            magic: Magic::V0,
        }
    }

    fn assert_roundtrip_i32(ver: Semver) {
        assert_eq!(Semver::from_i32(ver.to_i32().unwrap()).unwrap(), ver);
    }

    fn assert_roundtrip_u32(ver: Semver) {
        assert_eq!(Semver::from_u32(ver.to_u32().unwrap()).unwrap(), ver);
    }

    fn assert_roundtrip_i64(ver: Semver) {
        assert_eq!(Semver::from_i64(ver.to_i64().unwrap()).unwrap(), ver);
    }

    fn assert_roundtrip_u64(ver: Semver) {
        assert_eq!(Semver::from_u64(ver.to_u64().unwrap()).unwrap(), ver);
    }
}

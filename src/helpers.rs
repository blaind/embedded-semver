use bitvec::prelude::*;

use crate::{sizes, Error};

pub fn num_as_bv<const SIZE: usize, const ITER_SIZE: usize>(
    bv: &mut BitArray<Msb0, [u8; SIZE]>,
    iter: &mut sizes::SizeIterator<ITER_SIZE>,
    n: u64,
) -> Result<(), Error> {
    let range = iter.next().unwrap();
    if n > 2u64.pow(range.len() as u32) {
        Err(Error::Overflow)
    } else {
        bv[range].store(n);
        Ok(())
    }
}

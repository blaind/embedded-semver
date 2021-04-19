use core::ops::Range;

pub const I64_SIZES: [usize; 4] = [4, 16, 16, 16];
pub const I32_SIZES: [usize; 4] = [2, 10, 10, 10];

pub fn size_iterator<const SIZE: usize>(sizes: &'static [usize; SIZE]) -> SizeIterator<SIZE> {
    SizeIterator {
        sizes,
        idx: 0,
        previous_end: 0,
    }
}

pub struct SizeIterator<const SIZE: usize> {
    sizes: &'static [usize; SIZE],
    idx: usize,
    previous_end: usize,
}

impl<const SIZE: usize> Iterator for SizeIterator<SIZE> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.sizes.len() {
            let end = self.previous_end + self.sizes[self.idx];
            let range = self.previous_end..end;
            self.idx += 1;
            self.previous_end = end;

            Some(range)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut iterator = size_iterator(&I64_SIZES);
        assert_eq!(iterator.next(), Some(0..4));
        assert_eq!(iterator.next(), Some(4..20));
        assert_eq!(iterator.next(), Some(20..36));
        assert_eq!(iterator.next(), Some(36..52));
        assert_eq!(iterator.next(), None);
    }
}

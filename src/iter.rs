use crate::BitArray;

impl IntoIterator for BitArray {
    type IntoIter = BitArrayIter;
    type Item = bool;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BitArrayIter(self.0)
    }
}

pub struct BitArrayIter(u64);
impl Iterator for BitArrayIter {
    type Item = bool;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        self.0 >>= 1;
        Some(self.0 & 1 == 1)
    }
}

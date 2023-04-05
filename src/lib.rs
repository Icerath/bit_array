#![cfg_attr(not(feature = "std"), no_std)]

mod iter;
#[cfg(test)]
mod tests;

use core::fmt;
use core::ops::Index;

const TRUE: bool = true;
const FALSE: bool = false;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitArray(u64);

impl BitArray {
    const LEN: usize = 64;
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }
    #[inline]
    #[must_use]
    pub const fn get(&self, index: usize) -> Option<bool> {
        if index < 64 {
            Some(self.get_unchecked(index))
        } else {
            None
        }
    }
    /// ## Panics
    /// Panics if index >= 64.
    #[inline]
    pub fn set(&mut self, index: u8, value: bool) {
        if value {
            self.set_on(index);
        } else {
            self.set_off(index);
        }
    }
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        Self::LEN
    }
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    #[must_use]
    #[allow(clippy::similar_names)]
    pub const fn num_active(&self) -> usize {
        let mut num = self.0;
        let mut sum = 0;
        while num > 0 {
            sum += (num & 1 == 1) as usize;
            num >>= 1;
        }
        sum
    }
    /// ## Panics
    /// Panics if index >= 64.
    #[inline]
    pub fn set_on(&mut self, index: u8) {
        assert!(index < 64);
        self.0 |= 1 << index;
    }
    #[inline]
    /// ## Panics
    /// Panics if index >= 64.
    pub fn set_off(&mut self, index: u8) {
        assert!(index < 64);
        self.0 &= !(1 << index);
    }

    #[inline]
    pub const fn get_unchecked(&self, index: usize) -> bool {
        (((1u64 << index) & self.0) >> index) != 0
    }
}

impl From<u64> for BitArray {
    #[inline]
    #[must_use]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<BitArray> for u64 {
    #[inline]
    #[must_use]
    fn from(value: BitArray) -> Self {
        value.0
    }
}

impl From<[bool; 64]> for BitArray {
    fn from(value: [bool; 64]) -> Self {
        let mut array = Self::new();
        for index in 0..64u8 {
            let bool = value[index as usize];
            array.set(index, bool);
        }
        array
    }
}

#[allow(clippy::needless_range_loop)]
impl From<BitArray> for [bool; 64] {
    #[must_use]
    #[inline]
    fn from(val: BitArray) -> Self {
        let mut array = [false; 64];
        for index in 0..64 {
            array[index] = val.get_unchecked(index);
        }
        array
    }
}

impl fmt::Debug for BitArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:064b}", self.0)
    }
}

impl Index<usize> for BitArray {
    type Output = bool;
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 64);
        if self.get_unchecked(index) { &TRUE } else { &FALSE }
    }
}

use crate::index::indexing::{lowest_bit_increment, lowest_bit_removal};
use std::ops::AddAssign;

pub struct BinaryIndexTree<T> whe{
    n: usize,
    tree: Vec<T>,
}

impl BinaryIndexTree {
    pub fn new(length: usize) -> BinaryIndexTree {
        BinaryIndexTree {
            n: length + 1,
            tree: vec![0; length + 1],
        }
    }

    pub fn from(nums: &[i32]) -> BinaryIndexTree {
        let n = nums.len();
        let mut tree = Self::new(n);

        for i in 0..n {
            tree.update(i, nums[i]);
        }

        tree
    }

    /// Updates the original nums[i], which is zero-based
    /// which influences the sum of nums[..i+1]
    pub fn update(&mut self, index: usize, value: i32) {
        lowest_bit_increment(index, self.n).for_each(|x| self.tree[x] += value);
    }

    /// Query the sum of the original nums[..i]
    pub fn query(&self, i: usize) -> i32 {
        lowest_bit_removal(i).map(|x| self.tree[x]).sum()
    }
}

#[cfg(test)]
mod test {}

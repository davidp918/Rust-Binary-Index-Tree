use crate::index::indexing::{lowest_bit_increment, lowest_bit_removal};
use std::ops::AddAssign;

pub struct BinaryIndexTree<T>
where
    T: AddAssign + Copy + Default,
{
    n: usize,
    tree: Vec<T>,
}

impl<T> BinaryIndexTree<T>
where
    T: AddAssign + Copy + Default,
{
    pub fn new(length: usize) -> BinaryIndexTree<T> {
        BinaryIndexTree {
            n: length + 1,
            tree: vec![T::default(); length + 1],
        }
    }

    pub fn from(nums: &[T]) -> BinaryIndexTree<T> {
        let n = nums.len();
        let mut tree = Self::new(n);

        for i in 0..n {
            tree.update(i, nums[i]);
        }

        tree
    }

    /// Updates the original nums[i], which is zero-based
    /// which influences the sum of nums[..i+1]
    pub fn update(&mut self, index: usize, value: T) {
        lowest_bit_increment(index, self.n).for_each(|x| self.tree[x] += value);
    }

    /// Query the sum of the original nums[..i]
    pub fn query(&self, i: usize) -> T {
        let mut sum = T::default();
        for x in lowest_bit_removal(i) {
            sum += self.tree[x];
        }
        sum
    }
}

#[cfg(test)]
mod test {}

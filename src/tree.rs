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
            n: length,
            tree: vec![T::default(); length],
        }
    }

    pub fn from(nums: &[T]) -> BinaryIndexTree<T> {
        let n = nums.len() + 1;
        let mut tree = Self::new(n);

        for i in 1..n {
            tree.update(i, nums[i - 1]);
        }

        tree
    }

    /// Updates the original nums[i], which is zero-based
    /// which influences the sum of nums[..i+1]
    pub fn update(&mut self, index: usize, value: T) {
        for i in lowest_bit_increment(index, self.n) {
            self.tree[i] += value;
        }
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
mod test {
    use super::BinaryIndexTree;
    use rand::{thread_rng, Rng};

    #[test]
    fn update_test() {
        let mut rng = thread_rng();
        let nums: Vec<i32> = (0..5).map(|_| rng.gen_range(0..10)).collect();
        let tree = BinaryIndexTree::from(&nums);
        let prefix = prefix_sum(&nums);

        for i in 1..nums.len() {
            assert_eq!(tree.query(i), prefix[i]);
        }
    }

    fn prefix_sum(nums: &[i32]) -> Vec<i32> {
        Some(0)
            .iter()
            .chain(nums.iter().map(|x| *x).collect::<Vec<i32>>().iter())
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<i32>>()
    }
}

use crate::index::indexing::{increment_lsb, remove_lsb};
use std::ops::{AddAssign, Sub};

pub struct BinaryIndexTree<T>
where
    T: AddAssign + Sub<Output = T> + Copy + Default,
{
    n: usize,
    tree: Vec<T>,
}

impl<T> BinaryIndexTree<T>
where
    T: AddAssign + Sub<Output = T> + Copy + Default,
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
        let prev = self.query(index + 1) - self.query(index);
        let delta = value - prev;
        for i in increment_lsb(index + 1, self.n) {
            self.tree[i] += delta;
        }
    }

    /// Query the sum of the original nums[..i]
    pub fn query(&self, i: usize) -> T {
        let mut sum = T::default();
        for x in remove_lsb(i) {
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
    fn test() {
        let mut rng = thread_rng();
        let nums: Vec<i32> = (0..10).map(|_| rng.gen_range(0..10)).collect();
        let mut tree = BinaryIndexTree::from(&nums);
        let mut prefix = prefix_sum(&nums);

        assert_eq!(
            prefix,
            (0..tree.n)
                .into_iter()
                .map(|x| tree.query(x))
                .collect::<Vec<i32>>()
        );

        let new_value: i32 = rng.gen_range(0..10);
        let insertion_index: usize = rng.gen_range(0..10);

        for i in insertion_index..nums.len() {
            prefix[i + 1] -= nums[insertion_index];
            prefix[i + 1] += new_value;
        }
        tree.update(insertion_index, new_value);

        assert_eq!(
            prefix,
            (0..tree.n)
                .into_iter()
                .map(|x| tree.query(x))
                .collect::<Vec<i32>>()
        );
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

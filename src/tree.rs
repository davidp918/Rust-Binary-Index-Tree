use crate::index::indexing::{bit_remove, bit_up};
use std::ops::{AddAssign, Sub};

pub struct BinaryIndexTree<T>
where
    T: AddAssign + Sub<Output = T> + Copy + Default,
{
    n: usize,
    nums: Vec<T>,
}

impl<T> BinaryIndexTree<T>
where
    T: AddAssign + Sub<Output = T> + Copy + Default,
{
    /// Initialize a BIT filled with the default value of T
    pub fn new(length: usize) -> BinaryIndexTree<T> {
        BinaryIndexTree {
            n: length + 1,
            nums: vec![T::default(); length + 1],
        }
    }

    /// Construct a BIT from a given slice of T
    pub fn from(nums: &[T]) -> BinaryIndexTree<T> {
        let n = nums.len();
        let mut tree = Self::new(n);
        for i in 0..n {
            tree.update(i, nums[i]);
        }
        tree
    }

    /// Add delta to the original nums[index], runs faster than fn update_to
    /// since the difference of new and old value does not need to be calculated
    pub fn update(&mut self, index: usize, delta: T) {
        for i in bit_up(index + 1, self.n) {
            self.nums[i] += delta;
        }
    }

    /// Updates the original nums[index] to value, runs slower than fn update
    /// since the tree has to compute the difference of new and old value
    pub fn update_to(&mut self, index: usize, value: T) {
        let prev = self.query(index + 1) - self.query(index);
        let delta = value - prev;
        for i in bit_up(index + 1, self.n) {
            self.nums[i] += delta;
        }
    }

    /// Query the sum of the original nums[..index]
    /// Note that nums[index] is excluded
    pub fn query(&self, index: usize) -> T {
        let mut sum = T::default();
        for x in bit_remove(index) {
            sum += self.nums[x];
        }
        sum
    }

    /// Returns the sum over the given range, both inclusive,
    /// meaning that the original nums[left] & nums[right] are included in the sum
    pub fn range(&self, left: usize, right: usize) -> T {
        // right + 1, since the querying index is excluded
        self.query(right as usize + 1) - self.query(left as usize)
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
        tree.update_to(insertion_index, new_value);

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

pub mod indexing {
    use std::iter::successors;

    /// None inclusive limit, input should be tree.len()
    pub fn lowest_bit_increment(i: usize, limit: usize) -> impl Iterator<Item = usize> {
        assert!(i >= 1);
        assert!(i <= limit);
        assert!(limit <= (usize::max_value() >> 1));
        successors(Some(i), move |&i| {
            let next = increment(i);
            if next < limit {
                Some(next)
            } else {
                None
            }
        })
    }

    #[inline]
    fn increment(i: usize) -> usize {
        (i | i.wrapping_sub(1)).wrapping_add(1)
    }

    pub fn lowest_bit_removal(i: usize) -> impl Iterator<Item = usize> {
        assert!(i >= 1);
        successors(Some(i), move |&i| {
            let next = remove(i);
            if next > 0 {
                Some(next)
            } else {
                None
            }
        })
    }

    #[inline]
    fn remove(i: usize) -> usize {
        i & i.wrapping_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::indexing::*;

    #[test]
    fn removal_test() {
        let input: usize = 0b1101110101011010000;
        let res: Vec<usize> = vec![
            0b1101110101011010000,
            0b1101110101011000000,
            0b1101110101010000000,
            0b1101110101000000000,
            0b1101110100000000000,
            0b1101110000000000000,
            0b1101100000000000000,
            0b1101000000000000000,
            0b1100000000000000000,
            0b1000000000000000000,
        ];

        assert_eq!(res, lowest_bit_removal(input).collect::<Vec<usize>>());
    }

    #[test]
    fn increment_test() {
        let input: usize = 0b001101110101011010000;
        let limit: usize = 0b100000000000000000000;
        let res: Vec<usize> = vec![
            0b001101110101011010000,
            0b001101110101011100000,
            0b001101110101100000000,
            0b001101110110000000000,
            0b001101111000000000000,
            0b001110000000000000000,
            0b010000000000000000000,
            // 0b100000000000000000000,
        ];

        assert_eq!(
            res,
            lowest_bit_increment(input, limit).collect::<Vec<usize>>()
        )
    }
}

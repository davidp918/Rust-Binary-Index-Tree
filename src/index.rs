pub mod indexing {
    use std::iter::successors;
    pub fn lowest_bit_increment(i: usize, limit: usize) -> impl Iterator<Item = usize> {
        successors(Some(i), move |&i| {
            let next = increment(i);
            if i < limit {
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

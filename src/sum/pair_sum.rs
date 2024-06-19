use crate::prelude::types::Pair;

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::default::default_pair;
    #[test]
    fn test_pair_vector_sum() {
        assert_eq!(pair_vector_sum(default_pair(), default_pair()), (0, 0));
        assert_eq!(pair_vector_sum((1, 2), (3, 4)), (4, 6));
    }

    #[test]
    fn test_pair_scalar_sum() {
        assert_eq!(pair_scalar_sum((1, 2), (3, 4)), 10);
    }
}

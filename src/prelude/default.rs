use crate::prelude::types::{Pair, SignedCounter, UnsignedCounter, Vec3};

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn default_vec3() -> Vec3 {
    [0; 3]
}

pub fn default_pair() -> Pair {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_signed_counter() {
        assert_eq!(default_signed_counter(), 0);
    }

    #[test]
    fn test_default_vec3() {
        assert_eq!(default_vec3(), [0; 3]);
    }

    #[test]
    fn test_default_pair() {
        assert_eq!(default_pair(), (0, 0));
    }
}

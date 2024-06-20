pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_signed_counter() {
        assert_eq!(default_signed_counter(), 0);
    }

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(default_unsigned_counter(), 0);
    }

    #[test]
    fn test_prev_signed() {
        let counter: SignedCounter = 1;
        assert_eq!(prev_signed(counter), 0);
    }

    #[test]
    fn test_next_signed() {
        let counter: SignedCounter = 1;
        assert_eq!(prev_signed(counter), 0);
    }

    #[test]
    fn test_next_unsigned() {
        let counter: UnsignedCounter = 1;
        assert_eq!(next_unsigned(counter), 2)
    }
}

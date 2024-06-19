use crate::prelude::types::SignedCounter;

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;
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
}

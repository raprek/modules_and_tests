use crate::prelude::types::UnsignedCounter;

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_unsigned() {
        let counter: UnsignedCounter = 1;
        assert_eq!(next_unsigned(counter), 2)
    }
}

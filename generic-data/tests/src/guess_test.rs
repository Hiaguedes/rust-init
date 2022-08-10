#[cfg(test)]
pub mod guess_tests {
    use crate::guess::Counter;

    
    #[test]
      fn incrementa_counter() {
        let mut counter = Counter::new();

        (&mut counter).increment();

        assert_eq!(counter.get_value(), 1);
      }
}
#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;
    // use parameterized::parameterized;
   use test_case::test_case;
    
   
    #[test]
    fn test_get_sum_of_multiples() {
        let final_sum = get_sum_of_multiples(50);
        assert_eq!(final_sum, 543);
    }

}

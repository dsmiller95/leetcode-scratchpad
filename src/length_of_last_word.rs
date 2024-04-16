struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.chars()
            .into_iter()
            .rev()
            .skip_while(|c| c == &' ')
            .take_while(|c| c != &' ')
            .count() as i32
    }
}

mod test {
    use super::*;
    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }
}

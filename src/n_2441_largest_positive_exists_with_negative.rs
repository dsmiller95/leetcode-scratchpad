struct Solution();

use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct PairMatch {
    has_negative: bool,
    has_positive: bool,
}

impl PairMatch {
    fn is_matched(&self) -> bool {
        self.has_positive && self.has_negative
    }
}

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut num_table: HashMap<u16, PairMatch> = HashMap::with_capacity(nums.len() / 4);
        let mut max_out: Option<u16> = None;

        for num in nums {
            let num_abs = num.unsigned_abs() as u16;

            let existing_entry = num_table.entry(num_abs).or_default();
            match num {
                ..=-1 => {
                    existing_entry.has_negative = true;
                }
                1.. => {
                    existing_entry.has_positive = true;
                }
                _ => panic!("num must not equal 0"),
            }

            if !existing_entry.is_matched() {
                continue;
            }

            max_out = Some(match max_out {
                Some(x) => x.max(num_abs),
                None => num_abs,
            });
        }

        match max_out {
            Some(x) => x.into(),
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::n_2441_largest_positive_exists_with_negative::Solution;

    #[test]
    fn test_case_1() {
        let nums = [-1, 2, -3, 3];
        let expected = 3;

        let nums = nums.to_vec();
        assert_eq!(Solution::find_max_k(nums), expected)
    }

    #[test]
    fn test_case_2() {
        let nums = [-1, 10, 6, 7, -7, 1];
        let expected = 7;

        let nums = nums.to_vec();
        assert_eq!(Solution::find_max_k(nums), expected)
    }

    #[test]
    fn test_case_3() {
        let nums = [-10, 8, 6, 7, -2, -3];
        let expected = -1;

        let nums = nums.to_vec();
        assert_eq!(Solution::find_max_k(nums), expected)
    }

    #[test]
    fn test_case_4() {
        let nums = [-9, -43, 24, -23, -16, -30, -38, -30];
        let expected = -1;

        let nums = nums.to_vec();
        assert_eq!(Solution::find_max_k(nums), expected)
    }
}

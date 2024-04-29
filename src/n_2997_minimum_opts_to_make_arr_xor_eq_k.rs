struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut xor_all = nums
            .into_iter()
            .reduce(|acc, next_val| acc ^ next_val)
            // nums.length >= 1
            .unwrap();

        xor_all ^= k;

        xor_all.count_ones().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let nums = [2, 1, 3, 4];
        let k = 1;
        let expected = 2;

        assert_eq!(expected, Solution::min_operations(nums.to_vec(), k))
    }

    #[test]
    fn test_two() {
        let nums = [2, 0, 2, 0];
        let k = 0;
        let expected = 0;

        assert_eq!(expected, Solution::min_operations(nums.to_vec(), k))
    }
}

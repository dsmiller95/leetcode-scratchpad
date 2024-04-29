struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut xor_all = nums
            .into_iter()
            .reduce(|acc, next_val| acc ^ next_val)
            // nums.length >= 1
            .unwrap();

        xor_all ^= k;

        Solution::count_bits(xor_all).into()
    }

    fn count_bits(k: i32) -> u8 {
        let mut k: u32 = k.try_into().expect("k must be positive");
        let mut total: u8 = 0;
        while k != 0 {
            total += if k & 1 == 1 { 1 } else { 0 };
            k >>= 1;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use crate::n_2997_minimum_opts_to_make_arr_xor_eq_k::Solution;

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

    #[test]
    fn test_count_bits() {
        let num: i32 = 0b0101111101;
        let count = 7;
        assert_eq!(count, Solution::count_bits(num));
    }
}

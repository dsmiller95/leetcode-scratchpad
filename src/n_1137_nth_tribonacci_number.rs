struct Solution {}

const fn tribonacci_table<const TABLE_SIZE: usize>() -> [i32; TABLE_SIZE] {
    let mut result = [0; TABLE_SIZE];
    result[0] = 0;
    result[1] = 1;
    result[2] = 1;

    let mut i = 3;
    while i < TABLE_SIZE {
        result[i] = result[i - 3] + result[i - 2] + result[i - 1];
        i += 1;
    }
    result
}

const TRIB_TABLE: [i32; 38] = tribonacci_table();

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        TRIB_TABLE[n as usize]
    }
}

mod test {
    use crate::n_1137_nth_tribonacci_number::Solution;

    #[test]
    fn test_2nd_num_is_1() {
        let n = 2;
        let expected = 1;
        assert_eq!(expected, Solution::tribonacci(n));
    }

    #[test]
    fn test_4th_num() {
        let n = 4;
        let expected = 4;
        assert_eq!(expected, Solution::tribonacci(n));
    }

    #[test]
    fn test_25th_num() {
        let n = 25;
        let expected = 1389537;
        assert_eq!(expected, Solution::tribonacci(n))
    }
}

struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut last_3 = [0, 1, 1];
        if n <= 2 {
            return last_3[n as usize];
        }

        let mut next_n = 3;

        loop {
            let next_num = last_3.iter().sum();
            if next_n == n {
                return next_num;
            }

            last_3 = [last_3[1], last_3[2], next_num];
            next_n += 1;
        }
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

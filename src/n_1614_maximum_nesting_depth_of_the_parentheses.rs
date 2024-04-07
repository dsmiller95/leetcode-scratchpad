struct Solution{}

impl Solution {

    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0;
        let mut max_depth = 0;
        for char in s.chars() {
            depth += match char {
                '(' => 1,
                ')' => -1,
                _ => 0
            };
            max_depth = max_depth.max(depth);
        }

        max_depth
    }
}

mod test {

    use super::*;
    #[test]
    fn test_1(){
        assert_eq!(Solution::max_depth("()(())".to_string()), 2);
    }
    #[test]
    fn test_2(){
        assert_eq!(Solution::max_depth("".to_string()), 0);
    }
    #[test]
    fn test_3(){
        assert_eq!(Solution::max_depth("()()".to_string()), 1);
    }
    #[test]
    fn test_4(){
        assert_eq!(Solution::max_depth("(AB)(1 + (ABC))".to_string()), 2);
    }
}